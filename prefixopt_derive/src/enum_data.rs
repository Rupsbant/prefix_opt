use super::*;

pub fn derive(gen: &Generics, ident: &Ident, variant: &Vec<Variant>) -> quote::Tokens {
    let dummy = Ident::new(format!("_IMPL_PREFIXOPT_FOR_{}", ident));
    let mut collect = CollectVariants::empty();
    for var in variant {
        let one = OneVariant::build(var);
        collect.append_variant(ident, &var.ident, &one);
    }
    let impl_tokens = collect.finish(ident, gen);
    quote!(
        #[allow(non_upper_case_globals)]
        #[allow(unused_attributes, unused_imports, unused_variables)]
        const #dummy: () = {
            extern crate prefixopt;
            use prefixopt::*;
            use prefixopt::concat_ref::*;
            #impl_tokens
        };
    )
}
#[derive(Default)]
pub struct OneVariant {
    struct_fields: quote::Tokens,
    construct_concat: quote::Tokens,
    add_arguments: Vec<quote::Tokens>,
    fields_match: quote::Tokens,
    fields_tuple: quote::Tokens,
    override_arguments: quote::Tokens,
}

#[derive(Default)]
pub struct CollectVariants {
    struct_fields: quote::Tokens,
    construct_concat: quote::Tokens,
    add_groups: quote::Tokens,
    group_conflicts: quote::Tokens,
    add_arguments: quote::Tokens,
    override_arguments: quote::Tokens,
}
impl OneVariant {
    fn build(var: &Variant) -> OneVariant {
        let (f_ids, f_dots): (Vec<_>, Vec<_>) = var.data
            .fields()
            .iter()
            .enumerate()
            .map(|(i, ident)| {
                let display = ident
                    .ident
                    .as_ref()
                    .map(|id| id as &::std::fmt::Display)
                    .unwrap_or(&i);
                let id = Ident::new(format!("{}_{}", var.ident, display));
                let dot = format!("{}.{}", var.ident, display);
                (id, dot)
            })
            .unzip();
        let ty = var.data
            .fields()
            .iter()
            .map(|f| &f.ty)
            .collect::<Vec<_>>();
        let f_ids = &f_ids;
        let ty = &ty;
        let concat = quote!(#(#f_ids: <#ty as PrefixOpt>::Container::concat_prefix(&prefix.append(&#f_dots)),)*);
        let override_arguments = {
            let f_ids1 = f_ids;
            let f_ids2 = f_ids;
            let f_ids3 = f_ids;
            quote!(
                #(let #f_ids1 = if let Some(p) = self.#f_ids2.override_arguments(#f_ids3, matches) {
                    p} else {return None};)*)
        };
        let add_arguments = f_ids
            .iter()
            .map(|f_id| quote!(self.#f_id.as_arguments()))
            .collect::<Vec<_>>();
        let (tuple, matcher) = {
            let sep = quote!(#(#f_ids,)*);
            match var.data {
                VariantData::Struct(_) => {
                    let f = var.data
                        .fields()
                        .iter()
                        .map(|f| f.ident.as_ref().unwrap());
                    (quote!((#sep)), quote!({#(#f : #f_ids,)*}))
                }
                VariantData::Tuple(_) => (quote!((#sep)), quote!((#sep))),
                VariantData::Unit => (quote!(()), quote!()),
            }
        };
        OneVariant {
            struct_fields: quote!(#(#f_ids: <#ty as PrefixOpt>::Container,)*),
            construct_concat: concat,
            add_arguments: add_arguments,
            fields_match: matcher,
            fields_tuple: tuple,
            override_arguments: override_arguments,
        }
    }
}
impl CollectVariants {
    fn empty() -> CollectVariants {
        Default::default()
    }
    fn append_variant(&mut self, enu: &Ident, discriminant: &Ident, var: &OneVariant) {
        let group = Ident::new(format!("{}_group", discriminant));
        let discr_str = discriminant.as_ref();
        let group_str = group.as_ref();
        self.struct_fields.append(&var.struct_fields);
        self.struct_fields
            .append(quote!(
            #discriminant: String,
            #group: String,));
        self.construct_concat.append(&var.construct_concat);
        self.construct_concat
            .append(quote!(
            #discriminant: prefix.append(&#discr_str).into(),
            #group: prefix.append(&#group_str).into(),
        ));
        {
            let group_conflicts = &self.group_conflicts;
            self.add_groups
                .append(quote!(
            .add_group(
                clap::ArgGroup::with_name(&self.#group)
                    .multiple(true)
                    #group_conflicts)
        ));
        }
        self.group_conflicts
            .append(quote!(.conflicts_with(&self.#group)));
        let add_args = &var.add_arguments;
        {
            let group_rep = ::std::iter::repeat(&group);
            self.add_arguments
                .append(quote!(
            #(o.extend(#add_args.map_arg(|arg| arg.group(&self.#group_rep)));)*
            o.add_arg(clap::Arg::with_name(&self.#discriminant).long(&self.#discriminant).group(&self.#group));
        ));
        }
        let override_arguments = &var.override_arguments;
        let fields_match = &var.fields_match;
        let fields_tuple = &var.fields_tuple;
        self.override_arguments
            .append(quote!(
            if matches.is_present(&self.#group) {
                let #fields_tuple = match parsed {
                    #enu::#discriminant#fields_match => #fields_tuple,
                    _ => Default::default(),
                };
                #override_arguments;
                Some(#enu::#discriminant#fields_match)
            } else
        ))
    }
    fn finish(self, enum_name: &Ident, gen: &Generics) -> quote::Tokens {
        let ident_container = Ident::new(format!("PREFIXOPT_FOR_{}", enum_name));
        let (impl_generics, ty_generics, where_clause) = gen.split_for_impl();
        let struct_fields = self.struct_fields;
        let construct_concat = self.construct_concat;
        let add_groups = self.add_groups;
        let add_arguments = self.add_arguments;
        let override_arguments = self.override_arguments;
        quote!(
            #[allow(non_camel_case_types)]
            #[allow(non_snake_case)]
            pub struct #ident_container #impl_generics {
                #struct_fields
            }
            impl #impl_generics PrefixOptContainer for #ident_container
                    #ty_generics #where_clause {
                type Parsed = #enum_name #ty_generics;
                #[allow(non_snake_case)]
                fn concat_prefix(prefix: &ConcatRef<&Display>) -> Self {
                    Self {#construct_concat}
                }
                #[allow(non_snake_case)]
                fn as_arguments(&self) -> Args {
                    let mut o = Args::default()#add_groups;
                    #add_arguments;
                    o

                }
                #[allow(non_snake_case)]
                fn override_arguments(&self, parsed: Self::Parsed, matches: &clap::ArgMatches) -> Option<Self::Parsed> {
                    #override_arguments {
                        Some(parsed)
                  }
                }

            }
            impl #impl_generics PrefixOpt for #enum_name
                    #ty_generics #where_clause {
                type Container = #ident_container #ty_generics;
            }
        )
    }
}

use super::*;

pub fn derive(ident: &Ident, variant: &Vec<Variant>) -> quote::Tokens {
    let (idents, structs): (Vec<(_, _)>, Vec<_>) = variant
        .iter()
        .map(|var| {
            let struct_ident = Ident::new(format!("{}_{}", ident, var.ident));
            let decl_struct = decl_struct(&struct_ident, &var.data);
            let impl_prefix = super::variant_data::derive(&struct_ident, &var.data);
            let impl_from = impl_from(&ident, &var.ident, &struct_ident, &var.data);
            ((&var.ident, struct_ident),
             quote!(
                     #decl_struct
                     #impl_prefix
                     #impl_from
                 ))
        })
        .unzip();
    let ident_container = Ident::new(format!("PREFIXOPT_FOR_{}", ident));
    let decl_enum = decl_enum(&ident_container, &idents);
    let with_prefix = impl_with_prefix(&idents);
    let as_arguments = impl_as_arguments(&idents);
    let match_arguments = impl_match_arguments(&idents);
    let dummy = Ident::new(format!("_IMPL_PREFIXOPT_FOR_{}", ident));
    quote!(
        #[allow(non_upper_case_globals)]
        #[allow(unused_attributes, unused_imports, unused_variables)]
        const #dummy: () = {
            extern crate prefixopt;
            use prefixopt::*;
            use prefixopt::concat_ref::*;
            #decl_enum
            impl PrefixOptContainer for #ident_container {
                type Parsed = #ident;
                #with_prefix
                #as_arguments
                #match_arguments
            }
            impl PrefixOpt for #ident {
                type Container = #ident_container;
            }
            #(#structs)*
        };
    )
}

fn decl_enum(ident: &Ident, tags: &[(&Ident, Ident)]) -> quote::Tokens {
    let fname = tags.iter().map(|id| &id.0);
    let ftype = tags.iter().map(|id| &id.1);
    let group = tags.iter()
        .map(|&(ref id, _)| Ident::new(format!("{}_group", id)));
    quote!(
        #[allow(non_camel_case_types)]
        #[allow(non_snake_case)]
        #[derive(Debug)]
        pub struct #ident {
            #(
              #fname: <#ftype as PrefixOpt>::Container,
              #group: String,)*
        }
    )
}

fn impl_with_prefix(tags: &[(&Ident, Ident)]) -> quote::Tokens {
    let tname = tags.iter().map(|id| id.0);
    let ttype = tags.iter().map(|id| &id.1);
    let tfmts1 = tags.iter().map(|id| id.0.as_ref());
    let tfmts2 = tags.iter().map(|id| format!("{}_group", id.0));
    let group = tags.iter()
        .map(|id| Ident::new(format!("{}_group", id.0)));
    quote!(
        #[allow(non_snake_case)]
        fn concat_prefix(prefix: &ConcatRef<&Display>) -> Self {
            Self {
                #(#tname: <#ttype as PrefixOpt>::Container::concat_prefix(&prefix.append(&#tfmts1)),)*
                #(#group: prefix.append(&#tfmts2).into(),)*
            }
        }
    )
}
fn impl_as_arguments(tags: &[(&Ident, Ident)]) -> quote::Tokens {
    let group = tags.iter()
        .map(|id| Ident::new(format!("{}_group", id.0)))
        .collect::<Vec<_>>();
    let group_ref = &group;
    let (g1, g2):(Vec<_>, Vec<_>) = (0..group.len())
        .into_iter()
        .map(|idx: usize| {
            let subslice = &group_ref[0..idx.clone()];
            (&group_ref[idx], subslice.iter())
        })
        .unzip();
    let groups = &group;
    let t1 = tags.iter().map(|id| &id.0);
    quote!(
        fn as_arguments(&self) -> Args {
            let mut o = Args::default()#(
                .add_group(
                    clap::ArgGroup::with_name(&self.#g1)
                        .multiple(true)
                        #(.conflicts_with(&self.#g2))*
                )
            )*;
            #(o.extend(self.#t1.as_arguments().map_arg(|arg| arg.group(&self.#groups)));)*
            o
        }
    )
}
fn impl_match_arguments(tags: &[(&Ident, Ident)]) -> quote::Tokens {
    let tname = tags.iter().map(|id| id.0);
    let ttype = tags.iter().map(|id| &id.1);
    let group = tags.iter()
        .map(|id| Ident::new(format!("{}_group", id.0)))
        .collect::<Vec<_>>();
    quote!(
        fn override_arguments(&self, parsed: Self::Parsed, matches: &clap::ArgMatches) -> Option<Self::Parsed> {
            #(if matches.is_present(&self.#group) {
                let unwrap = #ttype::from(parsed);
                self.#tname.override_arguments(unwrap, matches).map(From::from)
            }else)* {
                Some(Self::Parsed::default())
            }

        }
    )
}

fn impl_from(enu: &Ident, name: &Ident, ty: &Ident, var_data: &VariantData) -> quote::Tokens {
    let (construct, matcher, match_constructor) = match *var_data {
        VariantData::Struct(ref v) => {
            let v1 = v.iter().map(|f| &f.ident);
            let v2 = v.iter().map(|f| &f.ident);;
            let construct = quote!({#(#v2: fr.#v1,)*});
            let v1 = v.iter().map(|f| &f.ident);
            let matcher = quote!({#(#v1,)*});
            (construct,
            matcher.clone(), matcher)
        }
        VariantData::Tuple(ref v) =>{
            let id = v.iter().enumerate().map(|(idx,_)| Ident::new(idx.to_string()));
            let construct = quote!((#(fr.#id,)*));
            let letters1 = v.iter().enumerate().map(|(idx,_)| Ident::new(format!("a{}", idx)));
            let matcher = quote!((#(#letters1,)*));
            (construct, matcher.clone(), matcher)
        }
        VariantData::Unit => (quote!(), quote!(), quote!(()))
    };
    quote!(
        impl From<#ty> for #enu {
            fn from(fr: #ty) -> Self {
                #enu::#name #construct
            }
        }
        impl #ty {
            fn from(fr: #enu) -> Self {
                match fr {
                    #enu::#name#matcher => #ty#match_constructor,
                    _ => #ty::default(),
                }
            }
        }
    )
}

fn decl_struct(ident: &Ident, variant_data: &VariantData) -> quote::Tokens {
    match *variant_data {
        VariantData::Struct(ref fields) => {
            let names = fields.iter().map(|f| f.ident.as_ref().unwrap());
            let types = fields.iter().map(|f| &f.ty);
            quote!(
                #[allow(non_camel_case_types)]
                #[derive(Debug, Default)]
                pub struct #ident{
                    #(#names: #types,)*
                }
            )
        }
        VariantData::Tuple(ref fields) => {
            let types = fields.iter().map(|f| &f.ty);
            quote!(
                #[allow(non_camel_case_types)]
                #[derive(Debug, Default)]
                pub struct #ident(#(#types,)*);
            )
        }
        VariantData::Unit => quote!(
            #[allow(non_camel_case_types)]
            #[derive(Debug, Default)]
            pub struct #ident();
        ),
    }
}

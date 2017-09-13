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
            use prefixopt::core::*;
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
    let tfmts = tags.iter().map(|id| format!("{{}}.{}", id.0));
    let prefix1 = tags.iter().map(|id| id.0);
    let prefix2 = tags.iter().map(|id| id.0);
    let prefix3 = tags.iter().map(|id| id.0);
    let group = tags.iter()
        .map(|id| Ident::new(format!("{}_group", id.0)));
    quote!(
        #[allow(non_snake_case)]
        fn with_prefix(prefix: &str) -> Self {
            #(let #prefix1 = format!(#tfmts, prefix);)*
            Self {
                #(#tname: <#ttype as PrefixOpt>::Container::with_prefix(&#prefix3),)*
                #(#group: #prefix2,)*
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
                        #(.conflict_with(&self.#g2))*
                )
            )*;
            #(o.extend(self.#t1.as_arguments().map_arg(|arg| arg.group(&self.#groups)));)*
            o
        }
    )
}
fn impl_match_arguments(tags: &[(&Ident, Ident)]) -> quote::Tokens {
    let types = tags.iter().map(|id| id.0);
    let group = tags.iter()
        .map(|id| Ident::new(format!("{}_group", id.0)))
        .collect::<Vec<_>>();
    quote!(
        fn match_arguments(&self, matches: &clap::ArgMatches) -> Option<Self::Parsed> {
            #(if matches.is_present(&self.#group) {
                self.#types.match_arguments(matches).map(From::from)
            }else)* {
                Some(Self::Parsed::default())
            }

        }
    )
}

fn impl_from(enu: &Ident, name: &Ident, ty: &Ident, var_data: &VariantData) -> quote::Tokens {
    let construct = match *var_data {
        VariantData::Struct(ref v) => {
            let v1 = v.iter().map(|f| &f.ident);
            let v2 = v.iter().map(|f| &f.ident);;
            quote!({#(#v2: fr.#v1,)*})
        }
        VariantData::Tuple(ref v) =>{
            let id = v.iter().enumerate().map(|(idx,_)| Ident::new(idx.to_string()));
            let q = quote!((#(fr.#id,)*));
            q
        }
        VariantData::Unit => quote!()
    };
    quote!(
        impl From<#ty> for #enu {
            fn from(fr: #ty) -> Self {
                #enu::#name #construct
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

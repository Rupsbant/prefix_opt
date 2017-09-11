use super::*;

pub fn derive(ident: &Ident, variant_data: &VariantData) -> quote::Tokens {
    let fields = match *variant_data {
        VariantData::Struct(ref v) => v,
        VariantData::Tuple(ref v) => v,
        VariantData::Unit => panic!("No unit structs or enums yet"),
    };
    let (names, types): (Vec<Ident>, Vec<_>) = fields
        .into_iter()
        .enumerate()
        .map(|(i, field): (usize, &Field)| {
                 let ident: Ident = field
                     .ident
                     .as_ref()
                     .cloned()
                     .unwrap_or_else(|| i.to_string().into());
                 (ident, field.ty.clone())
             })
        .unzip();
    let ident_container = Ident::new(format!("PREFIXOPT_FOR_{}", ident));
    let decl_struct = derive_struct(&ident_container, &names, &types);
    let constructor = derive_with_prefix(&names, &types);
    let builder = derive_as_arguments(&names);
    let matcher = derive_match_arguments(&names);
    let dummy = Ident::new(format!("_IMPL_PREFIXOPT_FOR_{}", ident));
    quote!(
        #[allow(non_upper_case_globals)]
        #[allow(unused_attributes, unused_imports, unused_variables)]
        const #dummy: () = {
            extern crate prefixopt;
            use prefixopt::core::*;
            #decl_struct
            impl PrefixOptContainer for #ident_container {
                type Parsed = #ident;
                #constructor
                #builder
                #matcher
            }
            impl PrefixOpt for #ident {
                type Container = #ident_container;
                fn with_prefix(prefix: &str) -> Self::Container {
                    Self::Container::with_prefix(prefix)
                }
            }
        };
    )
}
pub fn derive_struct(ident_container: &Ident, names: &Vec<Ident>, types: &Vec<Ty>) -> quote::Tokens {
    quote!(
        #[allow(non_camel_case_types)]
        #[derive(Debug)]
        pub struct #ident_container{
            #(
                #names: <#types as PrefixOpt>::Container,
            )*
        }
    )
}
pub fn derive_with_prefix(names: &Vec<Ident>, types: &Vec<Ty>) -> quote::Tokens {
    let names1 = names;
    let fmt = names.iter().map(|n| format!("{{}}.{}", n));
    quote!(
        fn with_prefix(prefix: &str) -> Self {
            Self {
                #(#names1: <#types as PrefixOpt>::Container::with_prefix(&format!(#fmt, prefix))),*
            }
        }
    )
}

pub fn derive_as_arguments(names: &Vec<Ident>) -> quote::Tokens {
    quote!(
        fn as_arguments(&self) -> Args {
            let mut args = Args::default();
            #(args.extend(self.#names.as_arguments()))*;
            args
        }
    )
}
pub fn derive_match_arguments(names: &Vec<Ident>) -> quote::Tokens {
    let names1 = names.iter();
    let names2 = names.iter();
    quote!(
        fn match_arguments(&self, matches: &clap::ArgMatches) -> Option<Self::Parsed> {
            let mut out = Self::Parsed::default();
            #(
                if let Some(p) = self.#names1.match_arguments(matches) {
                    out.#names2 = p;
                }
            )*
            Some(out)
        }
    )
}

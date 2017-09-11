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
    let constructor = derive_with_prefix(&names, &types);
    constructor
}

pub fn derive_with_prefix(names: &Vec<Ident>, types: &Vec<Ty>) -> quote::Tokens {
    let names1 = names;
    let names2 = names;
    quote!(
        fn with_prefix(prefix: &str) -> Self {
            Self {
                #(#names1: <#types as PrefixOptContainer>::with_prefix(&format!("{}.{}", prefix, #names2))),*
            }
        }
    )
}

pub fn derive_as_arguments(names: &Vec<Ident>) -> quote::Tokens {
    quote!(
        fn as_arguments(&self) -> clap::Args {
            let args = empty()
                #(.chain(#names.as_arguments()))*;
            args.collect::Vec<_>()
        }
    )
}
pub fn derive_match_arguments(names: &Vec<Ident>) -> quote::Tokens {
    panic!();
}

use super::*;

pub fn derive(ident: &Ident, variant_data: VariantData) -> quote::Tokens {
    let fields = match variant_data {
        VariantData::Struct(v) => v,
        VariantData::Tuple(v) => v,
        VariantData::Unit => panic!("No unit structs or enums yet"),
    };
    let fields = fields.into_iter().enumerate().map(|(i, mut field)|{
        field.ident = field.ident.or_else(|| Some(i.to_string().into()));
        field
    }).collect::<Vec<_>>();
    let constructor = quote!(
        PrefixOpt_#ident {(
            #fields.name: <#fields.ty as PrefixOptContainer>::with_prefix(&format!("{}.{}", prefix, #fields.name)),
        )*}
    );
    constructor
}

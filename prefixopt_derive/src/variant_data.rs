use super::*;

pub fn derive(ident: &Ident, variant_data: &VariantData) -> quote::Tokens {
    let dummy = Ident::new(format!("_IMPL_PREFIXOPT_FOR_{}", ident));
    let ident_container = Ident::new(format!("PREFIXOPT_FOR_{}", ident));
    let decl_struct = derive_struct(&ident_container, &variant_data);
    let fields = variant_data.fields();
    let constructor = derive_with_prefix(&ident_container, variant_data);
    let builder = derive_as_arguments(fields);
    let matcher = derive_override_arguments(fields);
    quote!(
        #[allow(non_upper_case_globals)]
        #[allow(unused_attributes, unused_imports, unused_variables)]
        const #dummy: () = {
            extern crate prefixopt;
            use prefixopt::*;
            use prefixopt::concat_ref::*;
            #decl_struct
            impl PrefixOptContainer for #ident_container {
                type Parsed = #ident;
                #constructor
                #builder
                #matcher
            }
            impl PrefixOpt for #ident {
                type Container = #ident_container;
            }
        };
    )
}
fn derive_struct(ident_container: &Ident, variant_data: &VariantData) -> quote::Tokens {
    match *variant_data {
        VariantData::Struct(ref fields) => {
            let names = fields.iter().map(|f| f.ident.as_ref().unwrap());
            let types = fields.iter().map(|f| &f.ty);
            quote!(
                #[allow(non_camel_case_types)]
                #[derive(Debug)]
                pub struct #ident_container{
                    #(#names: <#types as PrefixOpt>::Container,)*
                }
            )
        }
        VariantData::Tuple(ref fields) if fields.len() > 0 => {
            let types = fields.iter().map(|f| &f.ty);
            quote!(
                #[allow(non_camel_case_types)]
                #[derive(Debug)]
                pub struct #ident_container(#(<#types as PrefixOpt>::Container,)*);
            )
        }
        VariantData::Unit |
        VariantData::Tuple(_) => {
            quote!(
            #[allow(non_camel_case_types)]
            #[derive(Debug)]
            pub struct #ident_container(String);
        )
        }
    }
}
fn derive_with_prefix(ident_container: &Ident, variant_data: &VariantData) -> quote::Tokens {
    match *variant_data {
        VariantData::Struct(ref fields) => {
            let types = fields.iter().map(|f| &f.ty);
            let names = fields.iter().map(|f| f.ident.as_ref().unwrap());
            let fmt = fields
                .iter()
                .filter_map(|f| f.ident.as_ref())
                .map(|i| i.as_ref());
            quote!(
                fn concat_prefix(prefix: &ConcatRef<&Display>) -> Self {
                    Self {#(#names: <#types as PrefixOpt>
                        ::Container::concat_prefix(&prefix.append(&#fmt))),*}
                }
            )
        }
        VariantData::Tuple(ref fields) if fields.len() > 1 => {
            let types = fields.iter().map(|f| &f.ty);
            let fmt = fields
                .iter()
                .enumerate()
                .map(|(i, _)| i);
            quote!(
                fn concat_prefix(prefix: &ConcatRef<&Display>) -> Self {
                    #ident_container (#(<#types as PrefixOpt>
                        ::Container::concat_prefix(&prefix.append(&#fmt))),*)
                }
            )
        }
        VariantData::Tuple(ref fields) if fields.len() == 1 => {
            let types = &fields[0].ty;
            quote!(
                fn concat_prefix(prefix: &ConcatRef<&Display>) -> Self {
                    #ident_container (<#types as PrefixOpt>::Container::concat_prefix(prefix))
                }
            )
        }
        VariantData::Unit |
        VariantData::Tuple(_) => {
            quote!(
            fn concat_prefix(prefix: &ConcatRef<&Display>) -> Self {
                #ident_container(prefix.into())
            }
        )
        }
    }
}

fn derive_as_arguments(fields: &[Field]) -> quote::Tokens {
    let names = fields
        .iter()
        .enumerate()
        .map(|(i, f)| f.ident.as_ref().cloned().unwrap_or(Ident::new(i)));
    let unit_empty_tag = if fields.len() == 0 {
        quote!(args.add_arg(clap::Arg::with_name(&self.0).long(&self.0));)
    } else {
        quote!()
    };
    quote!(
        #[allow(unused_mut)]
        fn as_arguments(&self) -> Args {
            let mut args = Args::default();
            #(args.extend(self.#names.as_arguments());)*;
            #unit_empty_tag
            args
        }
    )
}

fn derive_override_arguments(fields: &[Field]) -> quote::Tokens {
    let names1 = fields
        .iter()
        .enumerate()
        .map(|(i, f)| f.ident.as_ref().cloned().unwrap_or(Ident::new(i)));
    let names2 = fields
        .iter()
        .enumerate()
        .map(|(i, f)| f.ident.as_ref().cloned().unwrap_or(Ident::new(i)));
    let names3 = fields
        .iter()
        .enumerate()
        .map(|(i, f)| f.ident.as_ref().cloned().unwrap_or(Ident::new(i)));
    quote!(
        #[allow(unused_mut)]
        fn override_arguments(&self, mut out: Self::Parsed, matches: &clap::ArgMatches) -> Option<Self::Parsed> {
            #(
                out.#names3 =
                if let Some(p) = self.#names1.override_arguments(out.#names2, matches) {p} else {return None};
            )*
            Some(out)
        }
    )
}

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::*;
mod enum_data;
mod variant_data;
mod generics;

/// Generates the `PrefixOpt` impl.
#[proc_macro_derive(PrefixOpt, attributes(prefixopt))]
pub fn prefixopt(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_prefixopt(ast);
    gen.parse().unwrap()
}

fn impl_prefixopt(ast: DeriveInput) -> quote::Tokens {
    let ref ident = ast.ident;
    let generics = generics::with_prefixopt_constraints(ast.generics);
    let tokens = match ast.body {
        syn::Body::Struct(_struct) => variant_data::derive(&generics, ident, &_struct),
        syn::Body::Enum(_enum) => enum_data::derive(&generics, ident, &_enum),
    };
    tokens
}

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::*;
mod generics;
mod enum_data;
mod variant_data;

use generics::add_prefix_opt;


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
    let generics = add_prefix_opt(&ast.generics);
    let tokens = match ast.body {
        syn::Body::Struct(_struct) => variant_data::derive(ident, &_struct),
        _ => panic!(),
    };
    panic!()
}

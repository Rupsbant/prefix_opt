extern crate clap;

#[macro_use]
extern crate prefixopt_derive;
extern crate prefixopt;
use prefixopt::core::*;

#[derive(Debug, PrefixOpt)]
pub enum A<T> {
    A(Box<T>, Option<Option<u8>>),
    B { x: T },
    C,
    D(),
    E(::std::marker::PhantomData<u32>),
}
impl<T> Default for A<T> {
    fn default() -> A<T> {
        A::C
    }
}

fn main() {
    let a_opt = A::<u32>::with_prefix("o");
    let app = a_opt.as_arguments().bind_app(clap::App::new("testing"));
    let parsed = a_opt.override_arguments(A::default(), &app.get_matches());
    println!("{:?}", parsed);
}

extern crate clap;

#[macro_use]
extern crate prefixopt_derive;
extern crate prefixopt;
use prefixopt::core::*;

#[derive(Debug, PrefixOpt)]
pub enum A<T> {
    A(Box<T>, Option<Option<u8>>),
    B{x:B},
    C,
    D(),
    E(::std::marker::PhantomData<u32>)
}

#[derive(Debug, PrefixOpt)]
pub enum B {
    Foo,
    Bar,
    Bux,
}
impl<T> Default for A<T> {
    fn default() -> A<T> {
        A::C
    }
}
impl Default for B {
    fn default() -> B {
        B::Foo
    }
}

fn main() {
    let splitc = A::<u32>::with_prefix("o");
    let args = splitc.as_arguments();
    let app = args.bind_app(clap::App::new("testing"));
    let matches = app.get_matches();
    let split = splitc.override_arguments(A::default(), &matches);
    println!("{:?}", split);
}

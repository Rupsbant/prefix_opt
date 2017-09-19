extern crate clap;

#[macro_use]
extern crate prefixopt_derive;
extern crate prefixopt;
use prefixopt::core::*;

#[derive(Debug, PrefixOpt)]
pub enum A {
    A(Box<u32>, Option<Option<u8>>),
    B(B),
    C,
    D(),
}

#[derive(Debug, PrefixOpt)]
pub enum B {
    Foo,
    Bar,
    Bux,
}
impl Default for A {
    fn default() -> A {
        A::A(Box::new(1), None)
    }
}
impl Default for B {
    fn default() -> B {
        B::Foo
    }
}

fn main() {
    let splitc = A::with_prefix("o");
    let args = splitc.as_arguments();
    let app = args.bind_app(clap::App::new("testing"));
    let matches = app.get_matches();
    let split = splitc.override_arguments(A::default(), &matches);
    println!("{:?}", split);
}

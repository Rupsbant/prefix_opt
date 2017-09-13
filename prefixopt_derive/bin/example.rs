extern crate clap;

#[macro_use]
extern crate prefixopt_derive;
extern crate prefixopt;
use prefixopt::core::*;

#[derive(Debug, PrefixOpt)]
pub enum A {
    A(u32, u16),
    B(B),
    C,
    D(),
}

#[derive(Debug, PrefixOpt)]
pub enum B {
    Foo,
    Bar(Option<Option<Bux>>),
    Bux(Box<Bux>),
}
#[derive(Debug, PrefixOpt, Default)]
pub struct Bux {
    name: String,
    age: u64,
}
impl Default for A {
    fn default() -> A {
        A::A(0, 0)
    }
}

impl Default for B {
    fn default() -> B {
        B::Foo
    }
}

fn main() {
    let splitc = A::with_prefix("o");
    println!("{:?}", splitc);
    let args = splitc.as_arguments();
    let app = args.bind_app(clap::App::new("testing"));
    let matches = app.get_matches();
    let split = splitc.match_arguments(&matches);
    println!("{:?}", split);
}

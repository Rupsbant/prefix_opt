extern crate clap;

#[macro_use]
extern crate prefixopt_derive;
extern crate prefixopt;
use prefixopt::core::*;

#[derive(Debug, PrefixOpt)]
pub enum A {
    A(u32, u16),
    B(u64),
    C,
    D()
}
impl Default for A {
    fn default() -> A {
        A::A(0,0)
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

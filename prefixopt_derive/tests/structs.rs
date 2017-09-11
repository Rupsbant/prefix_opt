
#[macro_use]
extern crate prefixopt_derive;

extern crate prefixopt;

#[test]
fn named_struct() {
    use prefixopt::core::PrefixOpt;
    #[derive(PrefixOpt, Default, Debug)]
    pub struct A {
        number: u64,
    }
    let ac = A::with_prefix("o");
    println!("{:?}", ac)
}

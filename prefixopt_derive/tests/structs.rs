
#[macro_use]
extern crate prefixopt_derive;

extern crate prefixopt;

#[test]
fn named_struct() {
    #[derive(PrefixOpt, Default, Debug)]
    pub struct A {
        number: u64,
    }
    panic!();
}

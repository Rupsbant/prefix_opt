
#[macro_use]
extern crate prefixopt_derive;

extern crate prefixopt;

#[test]
fn named_struct() {
    use prefixopt::core::PrefixOpt;
    #[derive(PrefixOpt, Default, Debug)]
    pub struct A {
        number: u64,
        b: u32,
        c: u16
    }
    let ac = A::with_prefix("o");
    println!("{:?}", ac)
}

#[test]
fn named_tuple() {
    use prefixopt::core::PrefixOpt;
    #[derive(PrefixOpt, Default, Debug)]
    pub struct A (u64,u32,u16);
    let ac = A::with_prefix("o");
    println!("{:?}", ac)
}


#[macro_use]
extern crate prefixopt_derive;

extern crate prefixopt;

#[test]
fn named_struct() {
    use prefixopt::core::PrefixOpt;
    #[derive(PrefixOpt, Debug)]
    pub enum A {
        A(u32, u32),
        B{x: u64, y: u64},
    }
    impl Default for A {
        fn default() -> Self {
            A::A(0,0)
        }
    }
}

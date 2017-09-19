
#[macro_use]
extern crate prefixopt_derive;
extern crate prefixopt;
extern crate clap;

#[test]
fn named_enum() {
    use prefixopt::*;
    #[derive(PrefixOpt, Debug, PartialEq, Eq)]
    pub enum A {
        A(u32, u32),
        B{x: u64, y: u64},
    }
    impl Default for A {
        fn default() -> Self {
            A::A(1,1)
        }
    }
    let a_opt = A::with_prefix("o");
    let app = a_opt.as_arguments().bind_app(clap::App::new("named_enum"));
    let matches = app.get_matches_from_safe(&["test", "--o.B.x=2"]);
    let a = a_opt.override_arguments(A::B{x:5,y:6}, &matches.unwrap());
    assert_eq!(a, Some(A::B{x:2, y:6}));
}

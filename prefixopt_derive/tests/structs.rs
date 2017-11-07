
#[macro_use]
extern crate prefixopt_derive;
extern crate prefixopt;
extern crate clap;

#[test]
fn named_struct() {
    use prefixopt::*;
    #[derive(PrefixOpt, Debug, PartialEq, Eq)]
    pub struct A {
        number: u64,
        b: u32,
        c: u16
    }
    let ac = A::with_prefix("o");
    let app = ac.as_arguments().bind_app(clap::App::new("named_enum"));
    let matches = app.get_matches_from_safe(&["test", "--o.number=2", "--o.b=3"]);
    let a = ac.override_arguments(A{number:1,b:2, c: 3}, &matches.unwrap()).unwrap();
    assert_eq!(a, A{number:2, b:3, c:3});
}

#[test]
fn named_tuple() {
    use prefixopt::*;
    #[derive(PrefixOpt, Debug, PartialEq, Eq)]
    pub struct A (u64,u32,u16);
    let ac = A::with_prefix("o");
    let app = ac.as_arguments().bind_app(clap::App::new("named_enum"));
    let matches = app.get_matches_from_safe(&["test", "--o.0=2", "--o.1=3"]);
    let a = ac.override_arguments(A(1, 2, 3), &matches.unwrap()).unwrap();
    assert_eq!(a, A(2, 3, 3));
}

#[test]
fn generic_struct() {
    use prefixopt::*;
    #[derive(PrefixOpt, Debug, PartialEq, Eq)]
    pub struct A<T> {
        a: T,
        b: T,
    }
    impl<T: Default> Default for A<T> {
        fn default() -> Self {
            A {
                a: T::default(),
                b: T::default(),
            }
        }
    }
    let a_opt = A::<u32>::with_prefix("o");
    let app = a_opt
        .as_arguments()
        .bind_app(clap::App::new("named_enum"));
    let matches = app.get_matches_from_safe(&["test", "--o.b=2"]);
    let a = a_opt.override_arguments(A { a: 5, b: 6 }, &matches.unwrap());
    assert_eq!(a, Some(A { a: 5, b: 2 }));
}

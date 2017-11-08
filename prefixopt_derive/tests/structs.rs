
#[macro_use]
extern crate prefixopt_derive;
extern crate prefixopt;
extern crate clap;

macro_rules! run_test {
    ( $ty:ty, $inp: expr, $( $args:expr),*) => {
        {
                let a_opt = <$ty as PrefixOpt>::with_prefix("o");
                let app = a_opt
                    .as_arguments()
                    .bind_app(clap::App::new("named_enum"));
                let matches = app.get_matches_from_safe(&["test", $($args,)*]);
                a_opt.override_arguments($inp, &matches.unwrap())
        }
    }
}

#[test]
fn named_struct() {
    use prefixopt::*;
    #[derive(PrefixOpt, Debug, PartialEq, Eq)]
    pub struct A {
        a: u64,
        b: u32,
        c: u16,
    }
    let a = run_test!(A, A { a: 1, b: 2, c: 3 }, "--o.a=2", "--o.b=3").unwrap();
    assert_eq!(a, A { a: 2, b: 3, c: 3 });
}

#[test]
fn named_tuple() {
    use prefixopt::*;
    #[derive(PrefixOpt, Debug, PartialEq, Eq)]
    pub struct A(u64, u32, u16);
    #[derive(PrefixOpt, Debug, PartialEq, Eq)]
    pub struct B(u64);
    let a = run_test!(A, A(1, 2, 3), "--o.0=2", "--o.1=3").unwrap();
    assert_eq!(a, A(2, 3, 3));
    let a = run_test!(B, B(1), "--o=2").unwrap();
    assert_eq!(a, B(2));
}

#[test]
fn fail_parse() {
    use prefixopt::*;
    #[derive(PrefixOpt, Debug, PartialEq, Eq)]
    pub struct A(u64, u32, u16);
    let a = run_test!(A, A(1, 2, 3), "--o.0=a");
    assert_eq!(a, None);
}

#[test]
fn generic_struct() {
    use prefixopt::*;
    #[derive(PrefixOpt, Default, Debug, PartialEq, Eq)]
    pub struct A<T> {
        a: T,
        b: T,
    }
    let a = run_test!(A<u32>, A { a: 1, b: 2}, "--o.a=2", "--o.b=3").unwrap();
    assert_eq!(a, A { a: 2, b: 3 });
}
#[test]
fn generic_tuple() {
    use prefixopt::*;
    #[derive(PrefixOpt, Debug, PartialEq, Eq)]
    pub struct A<T>(T, u32, u16);
    #[derive(PrefixOpt, Debug, PartialEq, Eq)]
    pub struct B<T>(T);
    let a = run_test!(A<f32>, A(1.,2,3), "--o.0=2", "--o.1=3").unwrap();
    assert_eq!(a, A(2.,3,3));
    let a = run_test!(B<f32>, B(1.), "--o=2").unwrap();
    assert_eq!(a, B(2.));
}

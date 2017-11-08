
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
mod simple {
    use prefixopt::*;
    #[derive(PrefixOpt, Debug, PartialEq, Eq)]
    pub enum A {
        A(u32, u32),
        B { x: u64, y: u64 },
        C(),
        D(u32),
        E,
    }
    impl Default for A {
        fn default() -> Self {
            A::A(1, 1)
        }
    }
    #[test]
    fn test_named_args_default() {
        let a = run_test!(A, A::default(), "--o.B.x=2").unwrap();
        assert_eq!(a, A::B { x: 2, y: 0 });
    }
    #[test]
    fn test_named_args_no_override() {
        let a = run_test!(A, A::B { x: 5, y: 6 }, "--o.B.x=2").unwrap();
        assert_eq!(a, A::B { x: 2, y: 6 });
    }
    #[test]
    fn test_empty_tuple() {
        let a = run_test!(A, A::B { x: 5, y: 6 }, "--o.C").unwrap();
        assert_eq!(a, A::C());
    }
    #[test]
    fn test_unary_tuple() {
        let a = run_test!(A, A::B { x: 5, y: 6 }, "--o.D.0=2").unwrap();
        assert_eq!(a, A::D(2));
    }
    #[test]
    fn test_unit() {
        let a = run_test!(A, A::B { x: 5, y: 6 }, "--o.E").unwrap();
        assert_eq!(a, A::E);
    }
}
mod generics {
    use prefixopt::*;
    #[derive(PrefixOpt, Debug, PartialEq, Eq)]
    pub enum Either<L, R> {
        Left { l: L },
        Right(R),
        None,
        Any(),
    }
    impl<L: Default, R> Default for Either<L, R> {
        fn default() -> Self {
            Either::Left { l: L::default() }
        }
    }
    #[test]
    fn generic_enum_named() {
        let a = run_test!(Either<u64,f64>, Either::default(), "--o.Left.l=2").unwrap();
        assert_eq!(a, Either::Left { l: 2 });
    }
    #[test]
    fn generic_unary() {
        let a = run_test!(Either<u64,f64>, Either::default(), "--o.Right.0=2.0").unwrap();
        assert_eq!(a, Either::Right(2.0));
    }
    #[test]
    fn generic_other() {
        let a = run_test!(Either<u64,f64>, Either::default(), "--o.None").unwrap();
        assert_eq!(a, Either::None);
        let a = run_test!(Either<u64,f64>, Either::default(), "--o.Any").unwrap();
        assert_eq!(a, Either::Any());
    }
}

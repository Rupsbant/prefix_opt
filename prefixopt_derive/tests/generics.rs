#[macro_use]
extern crate prefixopt_derive;
extern crate prefixopt;
extern crate clap;

#[test]
fn generic_enum() {
    use prefixopt::*;
    #[derive(PrefixOpt, Debug, PartialEq, Eq)]
    pub enum Either<L, R> {
        Left{l:L},
        Right(R),
        None,
        Any(),
    }
    impl<L: Default, R> Default for Either<L, R> {
        fn default() -> Self {
            Either::Left{l: L::default()}
        }
    }
    let a_opt = Either::<u64, f64>::with_prefix("o");
    let app = a_opt
        .as_arguments()
        .bind_app(clap::App::new("named_enum"));
    let matches = app.get_matches_from_safe(&["test", "--o.Right.0=2.0"]);
    let a = a_opt.override_arguments(Either::default(), &matches.unwrap());
    assert_eq!(a, Some(Either::Right(2.0)));
}

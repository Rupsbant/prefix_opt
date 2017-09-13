use super::core::*;
use std::marker::PhantomData;
use std::str::FromStr;

#[derive(Debug)]
pub struct Parsable<T>(String, PhantomData<T>);
impl<T: FromStr> PrefixOpt for T {
    type Container = Parsable<T>;
}

impl<T: FromStr> PrefixOptContainer for Parsable<T> {
    type Parsed = T;
    fn with_prefix(prefix: &str) -> Self {
        Parsable(prefix.into(), PhantomData)
    }
    fn as_arguments(&self) -> Args {
        Args(vec![clap::Arg::with_name(&self.0)
                      .long(&self.0)
                      .takes_value(true)],
             vec![])
    }
    fn match_arguments(&self, matches: &clap::ArgMatches) -> Option<Self::Parsed> {
        matches
            .value_of(&self.0)
            .map(str::parse)
            .and_then(Result::ok)
    }
}
pub struct Unit();

impl PrefixOptContainer for Parsable<Unit> {
    type Parsed = ();
    fn with_prefix(prefix: &str) -> Self {
        Parsable(prefix.into(), PhantomData)
    }
    fn as_arguments(&self) -> Args {
        Args(vec![clap::Arg::with_name(&self.0).long(&self.0)], vec![])
    }
    fn match_arguments(&self, _: &clap::ArgMatches) -> Option<Self::Parsed> {
        Some(())
    }
}

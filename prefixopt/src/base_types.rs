use super::core::*;

impl PrefixOpt for String {
    type Container = StringC;
    fn with_prefix(s: &str) -> Self::Container {
        StringC(s.into())
    }
}
pub struct StringC(String);
impl PrefixOptContainer for StringC {
    type Parsed = String;
    fn with_prefix(prefix: &str) -> Self {
        StringC(prefix.into())
    }
    fn as_arguments(&self) -> Args {
        Args(vec![clap::Arg::with_name(&self.0)
                  .long(&self.0)
                  .takes_value(true)],
         vec![])
    }
    fn match_arguments(&self, matches: &clap::ArgMatches) -> Option<Self::Parsed> {
        matches.value_of(&self.0).map(Into::into)
    }
}
pub struct U64C(String);
impl PrefixOpt for u64 {
    type Container = U64C;
    fn with_prefix(s: &str) -> Self::Container {
        U64C(s.into())
    }
}
impl PrefixOptContainer for U64C {
    type Parsed = u64;
    fn with_prefix(prefix: &str) -> Self {
        U64C(prefix.into())
    }
    fn as_arguments(&self) -> Args {
        Args(vec![clap::Arg::with_name(&self.0)
                  .long(&self.0)
                  .takes_value(true)],
         vec![])
    }
    fn match_arguments(&self, matches: &clap::ArgMatches) -> Option<Self::Parsed> {
        matches.value_of(&self.0).and_then(|v| v.parse().ok())
    }
}

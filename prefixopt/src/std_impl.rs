use super::*;

pub struct BoxContainer<T: PrefixOpt>(T::Container);

impl<T: PrefixOpt> PrefixOpt for Box<T> {
    type Container = BoxContainer<T>;
}
impl<T: PrefixOpt> PrefixOptContainer for BoxContainer<T> {
    type Parsed = Box<<T::Container as PrefixOptContainer>::Parsed>;

    fn with_prefix(prefix: &str) -> Self {
        BoxContainer(T::with_prefix(prefix))
    }
    fn as_arguments(&self) -> Args {
        self.0.as_arguments()
    }
    fn match_arguments(&self, matches: &clap::ArgMatches) -> Option<Self::Parsed> {
        self.0.match_arguments(matches).map(Box::new)
    }
}
pub struct OptionContainer<T: PrefixOpt> {
    some: T::Container,
    some_group: String,
}
impl<T: PrefixOpt> PrefixOpt for Option<T> {
    type Container = OptionContainer<T>;
}
impl<T: PrefixOpt> PrefixOptContainer for OptionContainer<T> {
    type Parsed = Option<<T::Container as PrefixOptContainer>::Parsed>;
    fn with_prefix(prefix: &str) -> Self {
        OptionContainer {
            some: T::with_prefix(prefix),
            some_group: format!("{}.some", prefix),
        }
    }
    fn as_arguments(&self) -> Args {
        let group = clap::ArgGroup::with_name(&self.some_group).multiple(true);
        let args = self.some
            .as_arguments()
            .add_group(group)
            .map_arg(|arg| arg.group(&self.some_group));
        args
    }
    fn match_arguments(&self, matches: &clap::ArgMatches) -> Option<Self::Parsed> {
        if matches.is_present(&self.some_group) {
            Some(self.some.match_arguments(matches))
        } else {
            None
        }
    }
}

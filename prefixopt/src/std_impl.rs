use super::*;

#[derive(Debug)]
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
#[derive(Debug)]
pub struct OptionContainer<T: PrefixOpt> {
    some: T::Container,
    some_group: String,
    none: String,
}
impl<T: PrefixOpt> PrefixOpt for Option<T> {
    type Container = OptionContainer<T>;
}
impl<T: PrefixOpt> PrefixOptContainer for OptionContainer<T> {
    type Parsed = Option<<T::Container as PrefixOptContainer>::Parsed>;
    fn with_prefix(prefix: &str) -> Self {
        let some = format!("{}.some", prefix);
        OptionContainer {
            some: T::with_prefix(&some),
            some_group: some,
            none: format!("{}.none", prefix),
        }
    }
    fn as_arguments(&self) -> Args {
        let group = clap::ArgGroup::with_name(&self.some_group).multiple(true);
        let mut args = self.some
            .as_arguments()
            .add_group(group)
            .map_arg(|arg| arg.group(&self.some_group));
        args.add_arg(clap::Arg::with_name(&self.none).long(&self.none));
        args
    }
    fn match_arguments(&self, matches: &clap::ArgMatches) -> Option<Self::Parsed> {
        if matches.is_present(&self.some_group){
            self.some.match_arguments(matches).map(Some)
        } else {
            Some(None)
        }
    }
}

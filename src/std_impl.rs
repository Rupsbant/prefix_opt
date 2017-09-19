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
    fn override_arguments(&self,
                          parsed: Self::Parsed,
                          matches: &clap::ArgMatches)
                          -> Option<Self::Parsed> {
        self.0
            .override_arguments(*parsed, matches)
            .map(Box::new)
    }
}
#[derive(Debug)]
pub struct OptionContainer<T: PrefixOpt> {
    some: T::Container,
    some_group: String,
    none: String,
}
impl<T: PrefixOpt> PrefixOpt for Option<T> where <T::Container as PrefixOptContainer>::Parsed: Default  {
    type Container = OptionContainer<T>;
}
impl<T: PrefixOpt> PrefixOptContainer for OptionContainer<T> where <T::Container as PrefixOptContainer>::Parsed: Default {
    type Parsed = Option<<T::Container as PrefixOptContainer>::Parsed>;
    fn with_prefix(prefix: &str) -> Self {
        let some = format!("{}.some", prefix);
        let some_g = format!("{}.some_g", prefix);
        OptionContainer {
            some: T::with_prefix(&some),
            some_group: some_g,
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
    fn override_arguments(&self,
                          parsed: Self::Parsed,
                          matches: &clap::ArgMatches)
                          -> Option<Self::Parsed> {
        if matches.is_present(&self.some_group) {
            let parsed = parsed.unwrap_or_default();
            self.some.override_arguments(parsed, matches).map(Some)
        } else if matches.is_present(&self.none) {
            Some(None)
        } else {
            Some(parsed)
        }
    }
}

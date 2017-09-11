pub extern crate clap;
extern crate map_in_place;

pub trait PrefixOpt {
    type Container: PrefixOptContainer;
    fn with_prefix(prefix: &str) -> Self::Container;
}

pub trait PrefixOptContainer {
    type Parsed;
    fn with_prefix(prefix: &str) -> Self;
    fn as_arguments(&self) -> Args;
    fn match_arguments(&self, matches: &clap::ArgMatches) -> Option<Self::Parsed>;
}

#[derive(Default)]
pub struct Args<'a: 'b, 'b>(pub Vec<clap::Arg<'a, 'b>>, pub Vec<clap::ArgGroup<'a>>);

impl<'a: 'b, 'b> Args<'a, 'b> {
    pub fn extend(&mut self, other: Args<'a, 'b>) {
        self.0.extend(other.0);
        self.1.extend(other.1);
    }
    pub fn add_group_mut(&mut self, g: clap::ArgGroup<'a>) {
        self.1.push(g)
    }
    pub fn add_group(mut self, g: clap::ArgGroup<'a>) -> Self {
        self.1.push(g);
        self
    }
    pub fn map_arg<F>(mut self, f: F) -> Self
        where F: FnMut(clap::Arg<'a, 'b>) -> clap::Arg<'a, 'b>
    {
        use self::map_in_place::*;
        self.0 = self.0.map_in_place(f);
        self
    }
    pub fn bind_app(self, clapp: clap::App<'a, 'b>) -> clap::App<'a, 'b> {
        clapp.groups(&self.1).args(&self.0)
    }
}

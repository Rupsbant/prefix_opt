use super::core::*;
use std::marker::PhantomData;
use std::str::FromStr;
use super::concat_ref::*;

#[derive(Debug)]
pub struct Parsable<T>(String, PhantomData<T>);
impl PrefixOpt for String {
    type Container = Parsable<String>;
}
impl PrefixOpt for bool {
    type Container = Parsable<bool>;
}
impl PrefixOpt for char {
    type Container = Parsable<char>;
}
impl PrefixOpt for u64 {
    type Container = Parsable<u64>;
}
impl PrefixOpt for u32 {
    type Container = Parsable<u32>;
}
impl PrefixOpt for u16 {
    type Container = Parsable<u16>;
}
impl PrefixOpt for u8 {
    type Container = Parsable<u8>;
}
impl PrefixOpt for usize {
    type Container = Parsable<usize>;
}
impl PrefixOpt for i64 {
    type Container = Parsable<i64>;
}
impl PrefixOpt for i32 {
    type Container = Parsable<i32>;
}
impl PrefixOpt for i16 {
    type Container = Parsable<i16>;
}
impl PrefixOpt for i8 {
    type Container = Parsable<i8>;
}
impl PrefixOpt for isize {
    type Container = Parsable<isize>;
}
impl PrefixOpt for f32 {
    type Container = Parsable<f32>;
}
impl PrefixOpt for f64 {
    type Container = Parsable<f64>;
}

impl<T: FromStr> PrefixOptContainer for Parsable<T> {
    type Parsed = T;
    fn concat_prefix(prefix: &ConcatRef<&Display>) -> Self {
        Parsable(prefix.into(), PhantomData)
    }
    fn as_arguments(&self) -> Args {
        Args(vec![clap::Arg::with_name(&self.0)
                      .long(&self.0)
                      .takes_value(true)],
             vec![])
    }
    fn override_arguments(&self,
                          parsed: Self::Parsed,
                          matches: &clap::ArgMatches)
                          -> Option<Self::Parsed> {
        matches
            .value_of(&self.0)
            .map(str::parse)
            .map(Result::ok)
            .unwrap_or(Some(parsed))
    }
}
pub struct Unit();

impl PrefixOptContainer for Parsable<Unit> {
    type Parsed = ();
    fn concat_prefix(prefix: &ConcatRef<&Display>) -> Self {
        Parsable(prefix.into(), PhantomData)
    }
    fn as_arguments(&self) -> Args {
        Args(vec![clap::Arg::with_name(&self.0).long(&self.0)], vec![])
    }
    fn override_arguments(&self, _: Self::Parsed, _: &clap::ArgMatches) -> Option<Self::Parsed> {
        Some(())
    }
}

pub struct Phantom<T: ?Sized>(::std::marker::PhantomData<T>);

impl<T: ?Sized> PrefixOpt for ::std::marker::PhantomData<T> {
    type Container = Parsable<Phantom<T>>;
}
impl<T: ?Sized> PrefixOptContainer for Parsable<Phantom<T>> {
    type Parsed = PhantomData<T>;
    fn concat_prefix(prefix: &ConcatRef<&Display>) -> Self {
        Parsable(prefix.into(), PhantomData)
    }
    fn as_arguments(&self) -> Args {
        Args(vec![], vec![])
    }
    fn override_arguments(&self, _: Self::Parsed, _: &clap::ArgMatches) -> Option<Self::Parsed> {
        Some(PhantomData)
    }
}

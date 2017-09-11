extern crate clap;

pub mod core;
pub mod base_types;

use core::*;
use base_types::*;

fn main() {
    let splitc = SplitC::with_prefix("o");
    let args = splitc.as_arguments();
    let app = args.bind_app(clap::App::new("testing"));
    let matches = app.get_matches();
    let split = splitc.match_arguments(&matches);
    println!("{:?}", split);
}


#[derive(Default, Debug)]
pub struct A {
    number: u64,
}
#[derive(Debug)]
pub enum Split {
    A(A),
    B(A, u64),
}

impl Default for Split {
    fn default() -> Split {
        Split::A(A { number: 1 })
    }
}
pub struct AC {
    number: U64C,
}
pub struct SplitC {
    ag: String,
    a: AC,
    bg: String,
    b0: AC,
    b1: U64C,
}
impl A {
    fn new(number: u64) -> A {
        A { number }
    }
}
impl PrefixOpt for A {
    type Container = AC;
    fn with_prefix(s: &str) -> AC {
        AC::with_prefix(s)
    }
}
impl PrefixOptContainer for AC {
    type Parsed = A;
    fn with_prefix(prefix: &str) -> Self {
        Self { number: u64::with_prefix(&format!("{}.number", prefix)) }
    }
    fn as_arguments(&self) -> Args {
        self.number.as_arguments()
    }
    fn match_arguments(&self, matches: &clap::ArgMatches) -> Option<Self::Parsed> {
        self.number.match_arguments(matches).map(A::new)
    }
}
impl PrefixOpt for Split {
    type Container = SplitC;
    fn with_prefix(s: &str) -> Self::Container {
        Self::Container::with_prefix(s)
    }
}
impl PrefixOptContainer for SplitC {
    type Parsed = Split;
    fn with_prefix(prefix: &str) -> Self {
        SplitC {
            ag: format!("{}.A", prefix),
            a: A::with_prefix(&format!("{}.A.0", prefix)),
            bg: format!("{}.B", prefix),
            b0: A::with_prefix(&format!("{}.B.0", prefix)),
            b1: u64::with_prefix(&format!("{}.B.1", prefix)),
        }
    }
    fn as_arguments(&self) -> Args {
        let ag = clap::ArgGroup::with_name(&self.ag).multiple(true);
        let bg = clap::ArgGroup::with_name(&self.bg).multiple(true).conflicts_with(&self.ag);
        let a = self.a.as_arguments();
        let b0 = self.b0.as_arguments();
        let b1 = self.b1.as_arguments();
        let mut o = Args::default().add_group(ag).add_group(bg);
        o.extend(a.map_arg(|arg| arg.group(&self.ag)));
        o.extend(b0.map_arg(|arg| arg.group(&self.bg)));
        o.extend(b1.map_arg(|arg| arg.group(&self.bg)));
        return o;
    }
    fn match_arguments(&self, matches: &clap::ArgMatches) -> Option<Self::Parsed> {
        if matches.is_present(&self.ag) {
            self.a.match_arguments(matches).map(Split::A)
        } else if matches.is_present(&self.bg) {
            let b0 = self.b0.match_arguments(matches).unwrap_or_default();
            let b1 = self.b1.match_arguments(matches).unwrap_or_default();
            Some(Split::B(b0, b1))
        } else {
            Some(Self::Parsed::default())
        }
    }
}

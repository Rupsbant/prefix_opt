extern crate clap;
extern crate prefixopt;

use prefixopt::*;
use parsable::*;
use concat_ref::*;

fn main() {
    let splitc = Split::with_prefix("o");
    let args = splitc.as_arguments();
    let app = args.bind_app(clap::App::new("testing"));
    let matches = app.get_matches();
    let split = splitc.override_arguments(Split::default(), &matches);
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
        Split::B(A { number: 5 }, 5)
    }
}
pub struct AC {
    number: Parsable<u64>,
}
pub struct SplitC {
    ag: String,
    a: AC,
    bg: String,
    b0: AC,
    b1: Parsable<u64>,
}
impl A {
    fn new(number: u64) -> A {
        A { number }
    }
}
impl PrefixOpt for A {
    type Container = AC;
}
impl PrefixOptContainer for AC {
    type Parsed = A;
    fn concat_prefix(prefix: &ConcatRef<&Display>) -> Self {
        Self { number: <u64 as PrefixOpt>::Container::concat_prefix(&prefix.append(&"number"))}
    }
    fn as_arguments(&self) -> Args {
        self.number.as_arguments()
    }
    fn override_arguments(&self, def: Self::Parsed, matches: &clap::ArgMatches) -> Option<Self::Parsed> {
        self.number.override_arguments(def.number, matches).map(A::new)
    }
}
impl PrefixOpt for Split {
    type Container = SplitC;
}
impl PrefixOptContainer for SplitC {
    type Parsed = Split;
    fn concat_prefix(prefix: &ConcatRef<&Display>) -> Self {
        SplitC {
            ag: prefix.append(&"A").into(),
            a: <A as PrefixOpt>::Container::concat_prefix(&prefix.append(&"A.0")),
            bg: prefix.append(&"B").into(),
            b0: A::with_prefix(&prefix.append(&"B.0")),
            b1: u64::with_prefix(&prefix.append(&"B.1")),
        }
    }
    fn as_arguments(&self) -> Args {
        let ag = clap::ArgGroup::with_name(&self.ag).multiple(true);
        let bg = clap::ArgGroup::with_name(&self.bg).multiple(true).conflicts_with(&self.ag);
        let all_groups = clap::ArgGroup::with_name("All").arg(&self.ag).arg(&self.bg);
        let a = self.a.as_arguments();
        let b0 = self.b0.as_arguments();
        let b1 = self.b1.as_arguments();
        let mut o = Args::default().add_group(ag).add_group(bg).add_group(all_groups);
        o.extend(a.map_arg(|arg| arg.group(&self.ag)));
        o.extend(b0.map_arg(|arg| arg.group(&self.bg)));
        o.extend(b1.map_arg(|arg| arg.group(&self.bg)));
        return o;
    }
    fn override_arguments(&self, def: Self::Parsed, matches: &clap::ArgMatches) -> Option<Self::Parsed> {
        macro_rules! try_opt {
            ($expression:expr) => (
                if let Some(p) = $expression {p} else {return None;}
            )
        }
        match (def, matches.is_present(&self.ag), matches.is_present(&self.bg)) {
            (Split::A(a), true, _) => self.a.override_arguments(a, matches).map(Split::A),
            (_, true, _) => self.a.override_arguments(A::default(), matches).map(Split::A),
            (Split::B(b0, b1), false, true) => {
                let b0 = try_opt!(self.b0.override_arguments(b0, matches));
                let b1 = try_opt!(self.b1.override_arguments(b1, matches));
                Some(Split::B(b0, b1))
            },
            (_, false, true) => {
                let b0 = try_opt!(self.b0.override_arguments(A::default(), matches));
                let b1 = try_opt!(self.b1.override_arguments(u64::default(), matches));
                Some(Split::B(b0, b1))
            },
            (_,_,_) => Some(Self::Parsed::default()),
        }
    }
}

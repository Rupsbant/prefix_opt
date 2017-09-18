# PrefixOpt
Add command line options with a prefix to override fields of structs and enums with a derive macro. This is made to extend clap.

## General use
Deriving PrefixOpt for a struct or enum implements the PrefixOpt trait for the struct which allows to create a struct with a prefix. This struct can be transformed into arguments and bound to a `clap::App`. The struct allows parsing the `ArgMatches` of the `App`.

Derivator of PrefixOpt requires that both structs and enums implement `Default`.

## Example

Add `prefixopt` and `prefixopt-derive` to your dependencies of your `Cargo.toml`:
```toml
[dependencies]
prefixopt = "0.1.0"
prefixopt-derive = "0.1.0"
```

And then, in your rust file:
```rust
extern crate prefixopt;
#[macro_use]
extern crate prefixopt_derive;
use prefixopt::core::*;

#[derive(Debug, PrefixOpt)]
pub enum A {
    A(Box<u32>, Option<Option<u8>>),
    B(B),
    C,
    D(),
}

#[derive(Debug, PrefixOpt)]
pub enum B {
    Foo,
    Bar,
    Bux,
}
impl Default for A {
    fn default() -> A {
        A::A(Box::new(1), None)
    }
}
impl Default for B {
    fn default() -> B {
        B::Foo
    }
}

fn main() {
    let splitc = A::with_prefix("o");
    let args = splitc.as_arguments();
    let app = args.bind_app(clap::App::new("testing"));
    let matches = app.get_matches();
    let split = splitc.match_arguments(&matches);
    println!("{:?}", split);
}
```

## Why
I have a program with too much options to encode manually. I liked OpenSSH's approach with `-o` which is hopefully automatically generated. I extended the approach to enums and liked the index-like syntax.

## To do and possible extensions
In a general order of personal importance.

* Fix bug regarding the default of an enum / struct not being the default of it's subtypes.
* Optional integration with structopt which is flat.
* Remove redundant indexing for enums, tuples and structs with a single field.
* Remove redundant formatting with a linked list of prefixes that is formatted once.
* Add annotation to remove dependency on default and require the arguments.
* Add annotation to change the printed name instead of the field name.
* Add annotation to alias short names or shorter long names for leaf values.
* Add annotation for descriptions to arguments.
* Remove with_prefix struct, depends on clap API change concerning owned strings, [issue 1041](https://github.com/kbknapp/clap-rs/issues/1041).

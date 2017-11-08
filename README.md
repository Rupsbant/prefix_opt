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
pub enum A<T> {
    A(Box<T>, Option<Option<u8>>),
    B{x:T},
    C,
    D(),
    E(::std::marker::PhantomData<u32>)
}
impl Default for A {
    fn default() -> A {
        A::A(Box::new(1), None)
    }
}

fn main() {
    let a_opt = A::<u32>::with_prefix("o");
    let app = a_opt.as_arguments().bind_app(clap::App::new("testing"));
    let a = a_opt.match_arguments(&app.get_matches());
    println!("{:?}", a);
}
```

## Why
I have a program with too much options to encode manually. I liked OpenSSH's approach with `-o` which is hopefully automatically generated. I extended the approach to enums and liked the index-like syntax.

## To do and possible extensions
In a general order of personal importance.

* Optional integration with structopt which is flat.
* Add annotation to remove dependency on default and require the arguments.
* Add annotation to change the printed name instead of the field name.
* Add annotation to alias short names or shorter long names for leaf values.
* Add annotation for descriptions to arguments.
* Remove with_prefix struct, depends on clap API change concerning owned strings, [issue 1041](https://github.com/kbknapp/clap-rs/issues/1041).

## Changelog

* 0.4.0 Adding generics for structs and enums.
* 0.3.0 Less formatting, removing redundant indexing for unary tuples in structs and enums.
* 0.2.0 Fixing default bug, an option minimally changes it's sisters: only for sister fields in a non-default enum discriminant.
* 0.1.0 Initial version, has known bugs.

## License
Dual-licensed.

Licensed under the Apache License, Version 2.0
http://www.apache.org/licenses/LICENSE-2.0 or the MIT license
http://opensource.org/licenses/MIT, at your
option. This project may not be copied, modified, or distributed
except according to those terms.

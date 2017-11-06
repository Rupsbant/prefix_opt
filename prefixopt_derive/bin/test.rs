
#[macro_use]
extern crate prefixopt_derive;
extern crate prefixopt;
extern crate clap;
fn main() {
    use prefixopt::*;
    #[derive(Debug, PartialEq, Eq)]
    pub enum Either<L, R> {
        Left(L),
        Right(R),
    }
    #[allow(non_upper_case_globals)]
    #[allow(unused_attributes, unused_imports, unused_variables)]
    const _IMPL_PREFIXOPT_FOR_Either: () = {
        extern crate prefixopt;
        use prefixopt::*;
        use prefixopt::concat_ref::*;
        #[allow(non_camel_case_types)]
        #[allow(non_snake_case)]
        pub struct PREFIXOPT_FOR_Either<L: PrefixOpt, R: PrefixOpt> {
            Left: <Either_Left<L, R> as PrefixOpt>::Container,
            Left_group: String,
            Right: <Either_Right<L, R> as PrefixOpt>::Container,
            Right_group: String,
        }
        impl<L: PrefixOpt, R: PrefixOpt> PrefixOptContainer for PREFIXOPT_FOR_Either<L, R>
            where <L as PrefixOpt>::Container: PrefixOptContainer<Parsed = L>,
                  <R as PrefixOpt>::Container: PrefixOptContainer<Parsed = R>
        {
            type Parsed = Either<L, R>;
            #[allow(non_snake_case)]
            fn concat_prefix(prefix: &ConcatRef<&Display>) -> Self {
                Self {
                    Left: <Either_Left as PrefixOpt>::concat_prefix(&prefix.append(&"Left")),
                    Right: <Either_Right as PrefixOpt>::concat_prefix(&prefix.append(&"Right")),
                    Left_group: prefix.append(&"Left_group").into(),
                    Right_group: prefix.append(&"Right_group").into(),
                }
            }
            fn as_arguments(&self) -> Args {
                let mut o = Args::default()
                    .add_group(clap::ArgGroup::with_name(&self.Left_group).multiple(true))
                    .add_group(clap::ArgGroup::with_name(&self.Right_group)
                                   .multiple(true)
                                   .conflicts_with(&self.Left_group));
                o.extend(self.Left
                             .as_arguments()
                             .map_arg(|arg| arg.group(&self.Left_group)));
                o.extend(self.Right
                             .as_arguments()
                             .map_arg(|arg| arg.group(&self.Right_group)));
                o
            }
            fn override_arguments(&self,
                                  parsed: Self::Parsed,
                                  matches: &clap::ArgMatches)
                                  -> Option<Self::Parsed> {
                if matches.is_present(&self.Left_group) {
                    let unwrap = Either_Left::from(parsed);
                    self.Left
                        .override_arguments(unwrap, matches)
                        .map(From::from)
                } else if matches.is_present(&self.Right_group) {
                    let unwrap = Either_Right::from(parsed);
                    self.Right
                        .override_arguments(unwrap, matches)
                        .map(From::from)
                } else {
                    Some(Self::Parsed::default())
                }
            }
        }
        impl<L: PrefixOpt, R: PrefixOpt> PrefixOpt for Either<L, R>
            where <L as PrefixOpt>::Container: PrefixOptContainer<Parsed = L>,
                  <R as PrefixOpt>::Container: PrefixOptContainer<Parsed = R>
        {
            type Container = PREFIXOPT_FOR_Either<L, R>;
        }
        #[allow(non_camel_case_types)]
        pub struct Either_Left<L, R>(L);
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl<L: ::std::fmt::Debug, R: ::std::fmt::Debug> ::std::fmt::Debug for Either_Left<L, R> {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    Either_Left(ref __self_0_0) => {
                        let mut builder = __arg_0.debug_tuple("Either_Left");
                        let _ = builder.field(&&(*__self_0_0));
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl<L: ::std::default::Default, R: ::std::default::Default> ::std::default::Default
            for Either_Left<L, R> {
            #[inline]
            fn default() -> Either_Left<L, R> {
                Either_Left(::std::default::Default::default())
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused_attributes, unused_imports, unused_variables)]
        const _IMPL_PREFIXOPT_FOR_Either_Left: () = {
            extern crate prefixopt;
            use prefixopt::*;
            use prefixopt::concat_ref::*;
            #[allow(non_camel_case_types)]
            pub struct PREFIXOPT_FOR_Either_Left<L: PrefixOpt,
                                                     R: PrefixOpt>(<L as
                                                                   PrefixOpt>::Container);
            impl<L: PrefixOpt, R: PrefixOpt> PrefixOptContainer for PREFIXOPT_FOR_Either_Left<L, R>
                where <L as PrefixOpt>::Container: PrefixOptContainer<Parsed = L>,
                      <R as PrefixOpt>::Container: PrefixOptContainer<Parsed = R>
            {
                type Parsed = Either_Left<L, R>;
                fn concat_prefix(prefix: &ConcatRef<&Display>) -> Self {
                    PREFIXOPT_FOR_Either_Left(<L as PrefixOpt>::concat_prefix(prefix))
                }
                #[allow(unused_mut)]
                fn as_arguments(&self) -> Args {
                    let mut args = Args::default();
                    args.extend(self.0.as_arguments());
                    args
                }
                #[allow(unused_mut)]
                fn override_arguments(&self,
                                      mut out: Self::Parsed,
                                      matches: &clap::ArgMatches)
                                      -> Option<Self::Parsed> {
                    out.0 = if let Some(p) = self.0.override_arguments(out.0, matches) {
                        p
                    } else {
                        return None;
                    };
                    Some(out)
                }
            }
            impl<L: PrefixOpt, R: PrefixOpt> PrefixOpt for Either_Left<L, R>
                where <L as PrefixOpt>::Container: PrefixOptContainer<Parsed = L>,
                      <R as PrefixOpt>::Container: PrefixOptContainer<Parsed = R>
            {
                type Container = PREFIXOPT_FOR_Either_Left<L, R>;
            }
        };
        impl<L: PrefixOpt, R: PrefixOpt> From<Either_Left<L, R>> for Either<L, R>
            where <L as PrefixOpt>::Container: PrefixOptContainer<Parsed = L>,
                  <R as PrefixOpt>::Container: PrefixOptContainer<Parsed = R>
        {
            fn from(fr: Either_Left<L, R>) -> Self {
                Either::Left(fr.0)
            }
        }
        impl<L: PrefixOpt, R: PrefixOpt> Either_Left<L, R>
            where <L as PrefixOpt>::Container: PrefixOptContainer<Parsed = L>,
                  <R as PrefixOpt>::Container: PrefixOptContainer<Parsed = R>
        {
            fn from(fr: Either<L, R>) -> Self {
                match fr {
                    Either::Left(a0) => Either_Left(a0),
                    _ => Either_Left::default(),
                }
            }
        }
        #[allow(non_camel_case_types)]
        pub struct Either_Right<L, R>(R);
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl<L: ::std::fmt::Debug, R: ::std::fmt::Debug> ::std::fmt::Debug for Either_Right<L, R> {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    Either_Right(ref __self_0_0) => {
                        let mut builder = __arg_0.debug_tuple("Either_Right");
                        let _ = builder.field(&&(*__self_0_0));
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl<L: ::std::default::Default, R: ::std::default::Default> ::std::default::Default
            for Either_Right<L, R> {
            #[inline]
            fn default() -> Either_Right<L, R> {
                Either_Right(::std::default::Default::default())
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused_attributes, unused_imports, unused_variables)]
        const _IMPL_PREFIXOPT_FOR_Either_Right: () = {
            extern crate prefixopt;
            use prefixopt::*;
            use prefixopt::concat_ref::*;
            #[allow(non_camel_case_types)]
            pub struct PREFIXOPT_FOR_Either_Right<L: PrefixOpt,
                                                      R: PrefixOpt>(<R as
                                                                    PrefixOpt>::Container);
            impl<L: PrefixOpt, R: PrefixOpt> PrefixOptContainer for PREFIXOPT_FOR_Either_Right<L, R>
                where <L as PrefixOpt>::Container: PrefixOptContainer<Parsed = L>,
                      <R as PrefixOpt>::Container: PrefixOptContainer<Parsed = R>
            {
                type Parsed = Either_Right<L, R>;
                fn concat_prefix(prefix: &ConcatRef<&Display>) -> Self {
                    PREFIXOPT_FOR_Either_Right(<R as PrefixOpt>::concat_prefix(prefix))
                }
                #[allow(unused_mut)]
                fn as_arguments(&self) -> Args {
                    let mut args = Args::default();
                    args.extend(self.0.as_arguments());
                    args
                }
                #[allow(unused_mut)]
                fn override_arguments(&self,
                                      mut out: Self::Parsed,
                                      matches: &clap::ArgMatches)
                                      -> Option<Self::Parsed> {
                    out.0 = if let Some(p) = self.0.override_arguments(out.0, matches) {
                        p
                    } else {
                        return None;
                    };
                    Some(out)
                }
            }
            impl<L: PrefixOpt, R: PrefixOpt> PrefixOpt for Either_Right<L, R>
                where <L as PrefixOpt>::Container: PrefixOptContainer<Parsed = L>,
                      <R as PrefixOpt>::Container: PrefixOptContainer<Parsed = R>
            {
                type Container = PREFIXOPT_FOR_Either_Right<L, R>;
            }
        };
        impl<L: PrefixOpt, R: PrefixOpt> From<Either_Right<L, R>> for Either<L, R>
            where <L as PrefixOpt>::Container: PrefixOptContainer<Parsed = L>,
                  <R as PrefixOpt>::Container: PrefixOptContainer<Parsed = R>
        {
            fn from(fr: Either_Right<L, R>) -> Self {
                Either::Right(fr.0)
            }
        }
        impl<L: PrefixOpt, R: PrefixOpt> Either_Right<L, R>
            where <L as PrefixOpt>::Container: PrefixOptContainer<Parsed = L>,
                  <R as PrefixOpt>::Container: PrefixOptContainer<Parsed = R>
        {
            fn from(fr: Either<L, R>) -> Self {
                match fr {
                    Either::Right(a0) => Either_Right(a0),
                    _ => Either_Right::default(),
                }
            }
        }
    };

    impl<L: Default, R> Default for Either<L, R> {
        fn default() -> Self {
            Either::Left(L::default())
        }
    }
    let a_opt = Either::<u64, f64>::with_prefix("o");
    let app = a_opt
        .as_arguments()
        .bind_app(clap::App::new("named_enum"));
    let matches = app.get_matches_from_safe(&["test", "--o.Right.0=2.0"]);
    let a = a_opt.override_arguments(Either::default(), &matches.unwrap());
    assert_eq!(a, Some(Either::Right(2.0)));

}

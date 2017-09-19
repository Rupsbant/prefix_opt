extern crate prefixopt;
extern crate clap;

#[test]
fn test_option_some() {
    use prefixopt::*;
    let ac = Option::<u64>::with_prefix("o");
    let app = ac.as_arguments().bind_app(clap::App::new("named_enum"));
    let matches = app.get_matches_from_safe(&["test", "--o.some=2"]);
    let a = ac.override_arguments(None, &matches.unwrap()).unwrap();
    assert_eq!(a, Some(2));
}
#[test]
fn test_option_none() {
    use prefixopt::*;
    let ac = Option::<u64>::with_prefix("o");
    let app = ac.as_arguments().bind_app(clap::App::new("named_enum"));
    let matches = app.get_matches_from_safe(&["test", "--o.none"]);
    let a = ac.override_arguments(Some(2), &matches.unwrap()).unwrap();
    assert_eq!(a,None);
}

#[test]
fn test_box() {
    use prefixopt::*;
    let ac = Box::<u64>::with_prefix("o");
    let app = ac.as_arguments().bind_app(clap::App::new("named_enum"));
    let matches = app.get_matches_from_safe(&["test", "--o=2"]);
    let a = ac.override_arguments(Box::new(5), &matches.unwrap()).unwrap();
    assert_eq!(a, Box::new(2));
}
#[test]
fn test_box_no_opt() {
    use prefixopt::*;
    let ac = Box::<u64>::with_prefix("o");
    let app = ac.as_arguments().bind_app(clap::App::new("named_enum"));
    let matches = app.get_matches_from_safe(&["test"]);
    let a = ac.override_arguments(Box::new(5), &matches.unwrap()).unwrap();
    assert_eq!(a, Box::new(5));
}

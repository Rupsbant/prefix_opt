use std::fmt;
pub use std::fmt::Display;

pub struct ConcatRef<'a, T: 'a + ?Sized> {
    before: Option<&'a ConcatRef<'a, T>>,
    after: T,
}

impl<'a, T: fmt::Display> fmt::Display for ConcatRef<'a, T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.before {
            Some(ref before) => write!(fmt, "{}.{}", before, self.after),
            None => write!(fmt, "{}", self.after),
        }
    }
}

impl<'a, T> ConcatRef<'a, T> {
    pub fn new_prefix(prefix: T) -> Self {
        ConcatRef {
            before: None,
            after: prefix,
        }
    }
    pub fn append(&'a self, postfix: T) -> ConcatRef<'a, T> {
        ConcatRef {
            before: Some(self),
            after: postfix,
        }
    }
}

impl<'a, T> From<T> for ConcatRef<'a, T> {
    fn from(fr: T) -> Self {
        Self::new_prefix(fr)
    }
}

impl<'a, T: fmt::Display> From<&'a ConcatRef<'a, T>> for String {
    fn from(fr: &ConcatRef<'a, T>) -> String {
        format!("{}", fr)
    }
}

impl<'a, T: fmt::Display> From<ConcatRef<'a, T>> for String {
    fn from(fr: ConcatRef<'a, T>) -> String {
        format!("{}", fr)
    }
}

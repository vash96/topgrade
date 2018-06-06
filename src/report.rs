use std::borrow::Cow;
use std::collections::HashMap;

pub type Report = HashMap<String, bool>;

pub trait Reporter {
    fn report<'a, M: Into<Cow<'a, str>>>(&self, key: M, report: &mut Report);
}

impl<T, E> Reporter for Result<T, E>
where
    T: Reporter,
{
    fn report<'a, M: Into<Cow<'a, str>>>(&self, key: M, report: &mut Report) {
        match self {
            Err(_) => {
                report.insert(key.into().into_owned(), false);
            }
            Ok(item) => {
                item.report(key, report);
            }
        }
    }
}

impl<T> Reporter for Option<T>
where
    T: Reporter,
{
    fn report<'a, M: Into<Cow<'a, str>>>(&self, key: M, report: &mut Report) {
        if let Some(item) = self {
            item.report(key, report);
        }
    }
}

impl Reporter for bool {
    fn report<'a, M: Into<Cow<'a, str>>>(&self, key: M, report: &mut Report) {
        report.insert(key.into().into_owned(), *self);
    }
}

impl Reporter for () {
    fn report<'a, M: Into<Cow<'a, str>>>(&self, key: M, report: &mut Report) {
        report.insert(key.into().into_owned(), true);
    }
}

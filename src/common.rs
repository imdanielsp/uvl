#[cfg(test)]
use mockall::{automock, mock, predicate::*};
#[cfg_attr(test, automock)]
pub trait ErrorReporter {
    fn report(&mut self, line: usize, location: &str, message: &str);

    fn error(&mut self, line: usize, message: &str);
}

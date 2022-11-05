pub trait ErrorReporter {
    fn report(&mut self, line: usize, location: &str, message: &str);

    fn error(&mut self, line: usize, message: &str);
}

use std::borrow::Cow;
use crate::int_stream::IntStream;

pub trait CharStream<'a>: IntStream<'a> {
    /// This method returns the text for the interval `start`..`end`.
    /// Text intercept as characters within this input stream.
    /// Guaranteed to not throw an exception
    fn text(&'a self, start: usize, end: usize) -> Cow<'a, str>;
}
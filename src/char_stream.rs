use std::borrow::Cow;
use crate::int_stream::IntStream;

/// [CharStream] is a stream type that can be indexed by one interval and return data as string.
pub trait CharStream<'a>: IntStream<'a> {
    /// This method returns the text for the interval `start`..`end`.
    /// Text intercept as characters within this input stream.
    /// Guaranteed to not throw an exception
    fn text(&'a self, start: usize, end: usize) -> Cow<'a, str>;
}
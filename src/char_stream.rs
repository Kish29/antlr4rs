use crate::int_stream::IntStream;

pub trait CharStream: IntStream {
    /// This method returns the text for the interval `start`..`end` of characters within this input stream.
    /// Guaranteed to not throw an exception
    fn text(&self, start: usize, end: usize) -> String;
}
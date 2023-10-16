use crate::int_stream::IntStream;

pub trait CharStream: IntStream {
    /// return the symbol of the interval `start`..`end`, include the symbol of `end`
    fn text(&self, start: usize, end: usize) -> String;
}
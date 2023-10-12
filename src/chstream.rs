use crate::int_stream::IntStream;
use crate::interval::Interval;

pub trait CharStream: IntStream {
    fn text(itv: &Interval) -> String;
}
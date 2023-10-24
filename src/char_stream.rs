use std::borrow::Cow;
use crate::input_stream::{ByteStream, CodePoint16BitStream, CodePoint32BitStream, CodePoint8BitStream, StringStream};
use crate::int_stream::IntStream;

/// [CharStream] is a stream type that can be indexed by one interval and return data as string.
pub trait CharStream: IntStream {
    /// This method returns the text for the interval `start`..`end`.
    /// Text intercept as characters within this input stream.
    /// Guaranteed to not throw an exception
    fn text(&self, start: usize, end: usize) -> Cow<'_, str>;
}

/// create a new [CharStream] from str.
pub fn from_str(s: &str) -> Box<dyn CharStream> {
    Box::new(StringStream::new(s.to_string()))
}

/// create a new [CharStream] from bytes.
pub fn from_bytes(b: Vec<u8>) -> Box<dyn CharStream> {
    Box::new(ByteStream::new(b))
}

/// create a new [CharStream] from code points 8bit.
pub fn from_code_point8bits(b: Vec<u8>) -> Box<dyn CharStream> {
    Box::new(CodePoint8BitStream::new(b))
}

/// create a new [CharStream] from code points 16bit.
pub fn from_code_point16bits(b: Vec<u16>) -> Box<dyn CharStream> {
    Box::new(CodePoint16BitStream::new(b))
}

/// create a new [CharStream] from code points 32bit.
pub fn from_code_point32bits(b: Vec<u32>) -> Box<dyn CharStream> {
    Box::new(CodePoint32BitStream::new(b))
}
use std::cmp::min;
use std::ops::Deref;
use crate::char_stream::CharStream;
use crate::code_point::CodePoints;
use crate::int_stream::{EOF, IntStream};

const INPUT_STREAM_SOURCE_NAME: &'static str = "source from string";

pub struct InputStream<T> {
    index: isize,
    size: isize,
    data: T,
}

type ByteStream<'a> = InputStream<&'a [u8]>;
type CodePoint8BitStream<'a> = InputStream<&'a [u8]>;
type CodePoint16BitStream<'a> = InputStream<&'a [u16]>;
type CodePoint32BitStream<'a> = InputStream<&'a [u32]>;

impl<'a, T: ?Sized + CodePoints> InputStream<&'a T> {
    pub fn new(input: &'a T) -> Self {
        Self {
            index: 0,
            size: input.size() as isize,
            data: input,
        }
    }
}

/// create new [InputStream] from str
pub fn from_str(s: &str) -> InputStream<&str> {
    InputStream::new(s)
}

/// create new [InputStream] from slice of byte
pub fn from_bytes(b: &[u8]) -> InputStream<&[u8]> {
    ByteStream::new(b)
}

/// create new [InputStream] from slice of u8, equal to from_bytes
pub fn from_u8s(u8s: &[u8]) -> InputStream<&[u8]> {
    CodePoint8BitStream::new(u8s)
}

/// create new [InputStream] from slice of u16
pub fn from_u16s(u16s: &[u16]) -> InputStream<&[u16]> {
    CodePoint16BitStream::new(u16s)
}

/// create new [InputStream] from slice of u32
pub fn from_u32s(u32s: &[u32]) -> InputStream<&[u32]> {
    CodePoint32BitStream::new(u32s)
}

impl<'a, T: Deref> IntStream for InputStream<T> where T::Target: CodePoints {
    #[inline]
    /// Consume(read) one char(rune)
    fn consume(&mut self) {
        // can not read EOF
        if self.size == self.index {
            panic!("can not consume EOF")
        }
        self.index += 1
    }

    /// Get the value of symbol at offset `i` from the current position.
    /// i >= 1 return the symbol ahead of current position, `EOF` will return if current position at the end of stream.
    /// return current symbol if `i` == 1
    /// i == 0 will panic cause the undefined operation
    /// i < 0 return the symbol back of current position, `EOF` will return if current position at the end of stream.
    /// eg: stream: abcdefg, current `index` is `2`
    /// stream.la(0) ===> panic
    /// stream.la(1) ===> int value of 'c'
    /// stream.la(2) ===> int value of 'd'
    /// stream.la(10) ===> EOF
    /// stream.la(-1) ===> int value of 'b'
    /// stream.la(-2) ===> int value of 'a'
    /// stream.la(-10) ===> EOF
    #[inline]
    fn la(&mut self, i: isize) -> isize {
        if i == 0 {
            panic!("undefined invocation: LA(0)")
        }
        // calculate offset
        let new_index;
        if i < 0 {
            new_index = self.index + i;
        } else {
            // case: i > 0, cause LA(1) return the current value in buffer, use index + i - 1;
            new_index = self.index + i - 1;
        }

        if new_index < 0 || new_index >= self.size {
            return EOF;
        }
        self.data.deref().code_point_at(new_index).unwrap_or(EOF)
    }

    /// mark/release do nothing; we have entire buffer
    #[inline]
    fn mark(&mut self) -> isize {
        -1
    }

    #[inline]
    fn release(&mut self, _marker: isize) {}

    #[inline]
    fn index(&self) -> isize {
        self.index
    }

    #[inline]
    fn seek(&mut self, index: isize) {
        if index <= self.index {
            self.index = index;
            return;
        }
        self.index = min(index, self.size)
    }

    #[inline]
    fn size(&self) -> isize {
        self.size
    }

    #[inline]
    fn source_name(&self) -> String {
        INPUT_STREAM_SOURCE_NAME.to_string()
    }
}

impl<'a, T: Deref> CharStream for InputStream<T> where T::Target: CodePoints {
    fn text(&self, start: usize, end: usize) -> String {
        self.data.deref().text_range(start, end)
    }
}
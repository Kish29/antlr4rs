use std::borrow::Cow;
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

pub type ByteStream<'a> = InputStream<&'a [u8]>;
pub type CodePoint8BitStream<'a> = InputStream<&'a [u8]>;
pub type CodePoint16BitStream<'a> = InputStream<&'a [u16]>;
pub type CodePoint32BitStream<'a> = InputStream<&'a [u32]>;

impl<'a, T: ?Sized + CodePoints<'a>> InputStream<&'a T> {
    pub fn new(input: &'a T) -> Self {
        Self {
            index: 0,
            size: input.size() as isize,
            data: input,
        }
    }
}

impl<'a, T: Deref> IntStream<'a> for InputStream<T> where T::Target: CodePoints<'a> {
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
    fn source_name(&'a self) -> Cow<'a, str> {
        Cow::Borrowed(INPUT_STREAM_SOURCE_NAME)
    }
}

impl<'a, T: Deref> CharStream<'a> for InputStream<T> where T::Target: CodePoints<'a> {
    #[inline]
    fn text(&'a self, start: usize, end: usize) -> Cow<'a, str> {
        self.data.deref().text_range(start, end)
    }
}
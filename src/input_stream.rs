use std::borrow::Cow;
use std::cmp::min;
use crate::char_stream::CharStream;
use crate::code_point::CodePoints;
use crate::int_stream::{EOF, IntStream};

const INPUT_STREAM_SOURCE_NAME: &'static str = "source from string";

/// [InputStream] is the implementation for [CharStream], only visual in lib.
/// we do not expect to export this struct to pub
pub struct InputStream<T> {
    index: isize,
    size: isize,
    data: T,
}

pub type StringStream = InputStream<String>;
pub type ByteStream = InputStream<Vec<u8>>;
pub type CodePoint8BitStream = InputStream<Vec<u8>>;
pub type CodePoint16BitStream = InputStream<Vec<u16>>;
pub type CodePoint32BitStream = InputStream<Vec<u32>>;

impl<T: CodePoints> InputStream<T> {
    /// returns a new [InputStream] and owned/clone the data from the `input`
    // #[inline(always)]
    pub fn new(input: T) -> Self {
        Self {
            index: 0,
            size: input.size() as isize,
            data: input,
        }
    }
}

impl<T: ToOwned + ?Sized> From<&T> for InputStream<T::Owned> where T::Owned: CodePoints {
    // #[inline(always)]
    fn from(input: &T) -> Self {
        let owned = input.to_owned();
        Self {
            index: 0,
            size: owned.size() as isize,
            data: owned,
        }
    }
}

impl<T: CodePoints> IntStream for InputStream<T> {
    // #[inline]
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
    // #[inline]
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
        if let Some(c) = self.data.code_point_at(new_index as usize) {
            return c as isize;
        }
        EOF
    }

    // #[inline]
    fn index(&self) -> isize {
        self.index
    }

    // #[inline]
    fn seek(&mut self, index: isize) {
        if index <= self.index {
            self.index = index;
            return;
        }
        self.index = min(index, self.size)
    }

    // #[inline]
    fn size(&self) -> isize {
        self.size
    }

    // #[inline]
    fn source_name(&self) -> Cow<'_, str> {
        Cow::Borrowed(INPUT_STREAM_SOURCE_NAME)
    }
}

impl<T: CodePoints> CharStream for InputStream<T> {
    // #[inline]
    fn text(&self, start: usize, end: usize) -> Cow<'_, str> {
        self.data.text_range(start, end)
    }
}
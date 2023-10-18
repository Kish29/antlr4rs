use std::cmp::min;
use std::ops::Deref;
use crate::char_stream::CharStream;
use crate::code_point::CodePoints;
use crate::int_stream::{EOF, IntStream};

pub struct InputStream<T> {
    name: String,
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
            name: "Data from string".to_string(),
            index: 0,
            size: input.len() as isize,
            data: input,
        }
    }
}

impl<'a, T: Deref> IntStream for InputStream<T> where T::Target: CodePoints {
    #[inline]
    /// Consume(read) one char(rune)
    fn consume(&mut self) {
        // can not read EOF
        if self.size == self.index {
            panic!("can not consume EOF")
        }
        self.index = self.index + 1
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
        self.data.code_point_at(new_index).unwrap_or(EOF)
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
        self.name.clone()
    }
}

impl<'a, T: Deref> CharStream<T> for InputStream<T> where T::Target: CodePoints {
    fn text(&self, start: usize, end: usize) -> String {
        todo!()
    }
}


mod tests {
    use crate::char_stream::CharStream;
    use crate::input_stream::{ByteStream, CodePoint16BitStream, CodePoint32BitStream, CodePoint8BitStream, InputStream};
    use crate::int_stream::{EOF, IntStream};

    #[test]
    fn test_input_stream() {
        let mut input = InputStream::new(r#"A你4好§，\❤"#);
        let input = &mut input as &mut dyn CharStream<&str>;
        assert_eq!(input.size(), 8);
        assert_eq!(input.la(1), 'A' as isize);
        assert_eq!(input.index(), 0);
        input.consume();
        assert_eq!(input.la(1), '你' as isize);
        assert_eq!(input.la(-1), 'A' as isize);
        assert_eq!(input.index(), 1);
        input.consume();
        assert_eq!(input.la(1), '4' as isize);
        assert_eq!(input.index(), 2);
        input.consume();
        assert_eq!(input.la(1), '好' as isize);
        assert_eq!(input.index(), 3);
        assert_eq!(input.la(-2), '你' as isize);
        input.consume();
        assert_eq!(input.la(1), '§' as isize);
        assert_eq!(input.index(), 4);
        assert_eq!(input.la(2), '，' as isize);
        assert_eq!(input.la(-2), '4' as isize);
        assert_eq!(input.la(3), '\\' as isize);
        input.consume();
        assert_eq!(input.la(1), '，' as isize);
        assert_eq!(input.index(), 5);
        assert_eq!(input.la(2), '\\' as isize);
        assert_eq!(input.la(-2), '好' as isize);
        assert_eq!(input.la(4), EOF);
        input.consume();
        assert_eq!(input.la(1), '\\' as isize);
        assert_eq!(input.index(), 6);
        assert_eq!(input.la(3), EOF);
        assert_eq!(input.la(-2), '§' as isize);
        assert_eq!(input.la(-10), EOF);
        input.consume();
        assert_eq!(input.la(1), '❤' as isize);
        assert_eq!(input.index(), 7);
        assert_eq!(input.la(2), EOF);
        assert_eq!(input.la(-3), '§' as isize);
        assert_eq!(input.la(-10), EOF);

        // assert_eq!(input.text(1, 1), "你");
        // assert_eq!(input.text(1, 2), "你4");
        // assert_eq!(input.text(3, 5), "好§，");
        // assert_eq!(input.text(0, 5), "A你4好§，");
        // assert_eq!(input.text(3, 10), "好§，\\❤");
    }

    #[test]
    fn test_byte_stream() {
        let mut v = ByteStream::new(&b"V\xaa\xbb"[..]);
        assert_eq!(v.la(1), 'V' as isize);
    }

    #[test]
    fn test_code_point_8bit_stream() {
        let mut v = CodePoint8BitStream::new(&b"V12"[..]);
        assert_eq!(v.la(1), 'V' as isize);
        assert_eq!(v.index(), 0);
        v.consume();
        assert_eq!(v.la(1), '1' as isize);
        assert_eq!(v.index(), 1);
        v.consume();
        assert_eq!(v.la(1), '2' as isize);
        assert_eq!(v.index(), 2);
    }

    #[test]
    fn test_code_point_16bit_stream() {
        let mut v = CodePoint16BitStream::new(&[0x00a3, 0x00a4, 0x00a5, 0x00a6, 0x00a7]);
        assert_eq!(v.la(1), '£' as isize);
        assert_eq!(v.index(), 0);
        v.consume();
        assert_eq!(v.la(1), '¤' as isize);
        assert_eq!(v.index(), 1);
        v.consume();
        assert_eq!(v.la(1), '¥' as isize);
        assert_eq!(v.index(), 2);
        v.consume();
        assert_eq!(v.la(1), '¦' as isize);
        assert_eq!(v.index(), 3);
        v.consume();
        assert_eq!(v.la(1), '§' as isize);
        assert_eq!(v.index(), 4);
        v.consume();
        assert_eq!(v.la(1), EOF);
    }

    #[test]
    fn test_code_point_32bit_stream() {
        let mut v = CodePoint32BitStream::new(&[0x00a3, 0x00a4, 0x00a5, 0x00a6, 0x00a7]);
        assert_eq!(v.la(1), '£' as isize);
        assert_eq!(v.index(), 0);
        v.consume();
        assert_eq!(v.la(1), '¤' as isize);
        assert_eq!(v.index(), 1);
        v.consume();
        assert_eq!(v.la(1), '¥' as isize);
        assert_eq!(v.index(), 2);
        v.consume();
        assert_eq!(v.la(1), '¦' as isize);
        assert_eq!(v.index(), 3);
        v.consume();
        assert_eq!(v.la(1), '§' as isize);
        assert_eq!(v.index(), 4);
        v.consume();
        assert_eq!(v.la(1), EOF);
    }
}
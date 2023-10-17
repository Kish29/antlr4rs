use std::cmp::min;
use crate::char_stream::CharStream;
use crate::int_stream::{EOF, IntStream};

pub struct InputStream {
    name: String,
    index: isize,
    size: isize,
    data: Vec<char>,
}

impl InputStream {
    pub fn new(text: &str) -> Box<Self> {
        let chars: Vec<char> = text.chars().collect();
        Box::new(Self {
            name: "Stream from string".to_string(),
            index: 0,
            size: chars.len() as isize,
            data: chars,
        })
    }
}

impl IntStream for InputStream {
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
    /// eg: stream: abcdefg, current index is `2`
    /// stream.la(0) ===> panic
    /// stream.la(1) ===> c
    /// stream.la(2) ===> d
    /// stream.la(10) ===> EOF
    /// stream.la(-1) ===> b
    /// stream.la(-2) ===> a
    /// stream.la(-10) ===> EOF
    fn la(&mut self, i: isize) -> isize {
        if i == 0 {
            panic!("undefined invocation: LA(0)")
        }
        // calculate offset
        let offset;
        if i < 0 {
            offset = self.index + i;
        } else {
            // case: i > 0, cause LA(1) return the current value in buffer, use index + i - 1;
            offset = self.index + i - 1;
        }
        if offset < 0 || offset >= self.size {
            EOF
        } else {
            self.data[offset as usize] as isize
        }
    }

    /// mark/release do nothing; we have entire buffer
    fn mark(&mut self) -> isize {
        -1
    }

    fn release(&mut self, _marker: isize) {}

    fn index(&self) -> isize {
        self.index
    }

    fn seek(&mut self, index: isize) {
        if index <= self.index {
            self.index = index;
            return;
        }
        self.index = min(index, self.size)
    }

    fn size(&self) -> isize {
        self.size
    }

    fn source_name(&self) -> String {
        self.name.clone()
    }
}

impl CharStream for InputStream {
    fn text(&self, start: usize, mut end: usize) -> String {
        if start > end {
            return "".to_string();
        }
        if end >= self.size as usize {
            end = (self.size - 1) as usize;
        }
        self.data[start..end + 1].into_iter().collect()
    }
}

mod tests {
    use std::ops::Index;
    use crate::char_stream::CharStream;
    use crate::input_stream::InputStream;
    use crate::int_stream::EOF;

    #[test]
    fn test_input_stream() {
        let text = "A你4好§，\\❤".to_string();
        let is = &mut *InputStream::new(&text) as &mut dyn CharStream;
        assert_eq!(is.la(1), 'A' as isize);
        assert_eq!(is.index(), 0);
        is.consume();
        assert_eq!(is.la(1), '你' as isize);
        assert_eq!(is.la(-1), 'A' as isize);
        assert_eq!(is.index(), 1);
        is.consume();
        assert_eq!(is.la(1), '4' as isize);
        assert_eq!(is.index(), 2);
        is.consume();
        assert_eq!(is.la(1), '好' as isize);
        assert_eq!(is.index(), 3);
        assert_eq!(is.la(-2), '你' as isize);
        is.consume();
        assert_eq!(is.la(1), '§' as isize);
        assert_eq!(is.index(), 4);
        assert_eq!(is.la(2), '，' as isize);
        assert_eq!(is.la(-2), '4' as isize);
        assert_eq!(is.la(3), '\\' as isize);
        is.consume();
        assert_eq!(is.la(1), '，' as isize);
        assert_eq!(is.index(), 5);
        assert_eq!(is.la(2), '\\' as isize);
        assert_eq!(is.la(-2), '好' as isize);
        assert_eq!(is.la(4), EOF);
        is.consume();
        assert_eq!(is.la(1), '\\' as isize);
        assert_eq!(is.index(), 6);
        assert_eq!(is.la(3), EOF);
        assert_eq!(is.la(-2), '§' as isize);
        assert_eq!(is.la(-10), EOF);
        is.consume();
        assert_eq!(is.la(1), '❤' as isize);
        assert_eq!(is.index(), 7);
        assert_eq!(is.la(2), EOF);
        assert_eq!(is.la(-3), '§' as isize);
        assert_eq!(is.la(-10), EOF);

        assert_eq!(is.text(1, 1), "你");
        assert_eq!(is.text(1, 2), "你4");
        assert_eq!(is.text(3, 5), "好§，");
        assert_eq!(is.text(0, 5), "A你4好§，");
        assert_eq!(is.text(3, 10), "好§，\\❤");
    }
}
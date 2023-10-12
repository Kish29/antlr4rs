use std::io::sink;
use std::thread::sleep;
use crate::chstream::CharStream;
use crate::int_stream::IntStream;
use crate::interval::Interval;

pub struct CodePointCharStream<T> {
    name: String,
    index: isize,
    size: isize,
    data: T,
}

pub type ByteStream<'a> = CodePointCharStream<&'a [u8]>;
pub type CodePoint8BitCharStream<'a> = CodePointCharStream<&'a [u8]>;
pub type CodePoint16BitCharStream<'a> = CodePointCharStream<&'a [u16]>;
pub type CodePoint32BitCharStream<'a> = CodePointCharStream<&'a [u32]>;

impl<'a> IntStream for ByteStream<'a> {
    // consume(read) one rune(8 bit)
    fn consume(&mut self) {
        // can not read EOF
        if self.size == self.index {
            panic!("can not consume EOF")
        }
        self.index = self.index + 1
    }

    fn la(&mut self, i: isize) -> isize {
        todo!()
    }

    fn mark(&mut self) -> isize {
        todo!()
    }

    fn release(&mut self, marker: isize) {
        todo!()
    }

    fn index() -> isize {
        todo!()
    }

    fn seek(&mut self, index: isize) {
        todo!()
    }

    fn size() -> isize {
        todo!()
    }

    fn source_name() -> String {
        todo!()
    }
}
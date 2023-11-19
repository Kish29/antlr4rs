use crate::recognizer::Recognizer;

pub trait Parser: Recognizer {
    fn precedence(&self) -> isize;
}
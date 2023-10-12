use std::rc::Rc;
use crate::token::Token;

pub trait TokenSource {
    fn next_token() -> Option<Rc<dyn Token>>;

    fn line() -> isize;

    fn char_position_in_line() -> isize;

    fn input_stream() -> isize;
}
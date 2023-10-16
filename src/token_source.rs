use std::rc::Rc;
use crate::token::Token;

pub trait TokenSource {
    fn next_token(&mut self) -> Option<Rc<dyn Token>>;

    fn line(&self) -> isize;

    fn char_position_in_line(&self) -> isize;

    fn input_stream(&mut self) -> isize;
}
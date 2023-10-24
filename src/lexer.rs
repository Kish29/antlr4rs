use crate::recognizer::Recognizer;
use crate::token::Token;
use crate::token_source::TokenSource;

pub trait Lexer: TokenSource + Recognizer {
    type TK: Token + Clone + ?Sized;

    fn emit(&mut self) -> Self::TK;
}

pub struct BaseLexer {}
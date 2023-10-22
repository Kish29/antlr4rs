use crate::recognizer::Recognizer;
use crate::token::Token;
use crate::token_source::TokenSource;

pub trait Lexer<'a>: TokenSource<'a> + Recognizer<'a> {
    type TK: Token<'a> + Clone + ?Sized;

    fn emit(&mut self) -> Self::TK;
}

pub struct BaseLexer {}
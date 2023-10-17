use crate::recognizer::Recognizer;
use crate::token_source::TokenSource;

pub trait Lexer: TokenSource + Recognizer {}

pub struct BaseLexer {}
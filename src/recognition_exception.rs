use std::borrow::Cow;
use crate::int_stream::IntStream;
use crate::token::Token;

pub trait RecognitionException {
    fn offending_token<TK: Token>(&self) -> &TK;

    fn message(&self) -> Cow<'_, str>;

    fn input_stream<I: IntStream>(&self) -> &I;
}
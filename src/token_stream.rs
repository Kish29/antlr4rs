use std::borrow::Cow;
use std::rc::Rc;
use crate::int_stream::IntStream;
use crate::token::Token;

pub trait TokenStream: IntStream {
    type TK: Token + Clone + ?Sized;

    fn lt(&self) -> Rc<Self::TK>;

    fn token_at(&self, idx: usize) -> Rc<Self::TK>;

    fn all_text(&self) -> Cow<'_, str>;


}
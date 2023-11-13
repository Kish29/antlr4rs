use std::sync::atomic::AtomicIsize;
use crate::char_stream::CharStream;
use crate::token::{BaseToken, Token};

pub trait TokenFactory {
    type TK: Token + Clone + ?Sized;

    fn create<S>(
        &self,
        stream: &S,
        token_type: isize,
        text: Option<String>,
        channel: isize,
        start: isize,
        stop: isize,
        line: isize,
        column: isize,
    ) -> Self::TK where S: CharStream;
}

pub struct CommonTokenFactory;

impl Default for CommonTokenFactory {
    fn default() -> Self {
        Self {}
    }
}

impl TokenFactory for CommonTokenFactory {
    type TK = BaseToken;

    #[inline]
    fn create<S>(
        &self,
        stream: &S,
        token_type: isize,
        text: Option<String>,
        channel: isize,
        start: isize,
        stop: isize,
        line: isize,
        column: isize,
    ) -> Self::TK where S: CharStream {
        BaseToken::new(
            token_type,
            channel,
            start,
            stop,
            AtomicIsize::new(-1),
            line,
            column,
            if let Some(t) = text { t } else {
                stream.text(start as usize, stop as usize).to_string()
            },
            false,
        )
    }
}
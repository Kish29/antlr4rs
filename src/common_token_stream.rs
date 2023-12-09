use std::rc::Rc;
use crate::token_source::TokenSource;

// CommonTokenStream is an implementation of TokenStream that loads tokens from
// a TokenSource on-demand and places the tokens in a buffer to provide access
// to any previous token by index. This token stream ignores the value of
// Token.getChannel. If your parser requires the token stream filter tokens to
// only those on a particular channel, such as Token.DEFAULT_CHANNEL or
// Token.HIDDEN_CHANNEL, use a filtering token stream such a CommonTokenStream.
pub struct CommonTokenStream<TS: TokenSource> {
    pub(crate) token_source: TS,

    pub(crate) channel: isize,
    pub(crate) tokens: Vec<Rc<<TS as TokenSource>::TK>>,
}

impl<TS: TokenSource> CommonTokenStream<TS> {
    // #[inline(always)]
    pub fn new(lexer: TS, channel: isize) -> Self {
        Self {
            channel,
            tokens: vec![],
            token_source: lexer,
        }
    }
}
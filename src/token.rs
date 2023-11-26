use std::borrow::Cow;
use std::fmt::{Debug, Display, Formatter};
use std::sync::atomic::{AtomicIsize, Ordering};
use crate::int_stream;

/// A token has properties: text, type, line, character position in the line
///  (so we can ignore tabs), token channel, index, and source from which
///  we obtained this token.
pub const TOKEN_INVALID_TYPE: isize = 0;
pub const TOKEN_EPSILON: isize = -2;
pub const TOKEN_MIN_USER_TOKEN_TYPE: isize = 1;
pub const TOKEN_EOF: isize = int_stream::EOF;
pub const TOKEN_DEFAULT_CHANNEL: isize = 0;
pub const TOKEN_HIDDEN_CHANNEL: isize = 1;
pub const TOKEN_MIN_USER_CHANNEL_VALUE: isize = 2;

const TEXT_EOF: &'static str = "<EOF>";

pub trait Token: Debug + Display {
    /// Get the type of the token */
    fn token_type(&self) -> isize;

    /// Return the channel of this token. Each token can arrive at the parser
    /// on a different channel, but the parser only "tunes" to a single channel.
    /// The parser ignores everything not on DEFAULT_CHANNEL.
    fn channel(&self) -> isize { TOKEN_DEFAULT_CHANNEL }

    /// The starting character index of the token
    ///  This method is optional; return -1 if not implemented.
    fn start(&self) -> isize { 0 }

    /// The last character index of the token.
    ///  This method is optional; return -1 if not implemented.
    fn stop(&self) -> isize { 0 }

    /// The line number on which the 1st character of this token was matched,
    ///  line=1..n
    fn line(&self) -> isize { 0 }

    fn column(&self) -> isize { 0 }

    /// Get the text of the token.
    fn text(&self) -> Cow<'_, str>;

    /// An index from 0..n-1 of the token object in the input stream.
    /// This must be valid in order to print token streams and
    /// use TokenRewriteStream.
    /// Return -1 to indicate that this token was conjured up since
    /// it doesn't have a valid index.
    fn token_index(&self) -> isize { 0 }

    fn set_token_index(&self, idx: isize);
}

#[derive(Debug)]
pub struct BaseToken {
    token_type: isize,
    channel: isize,
    start: isize,
    stop: isize,
    token_index: AtomicIsize,
    line: isize,
    column: isize,
    text: String,
    read_only: bool,
}

impl BaseToken {
    #[inline(always)]
    pub fn new(
        token_type: isize,
        channel: isize,
        start: isize,
        stop: isize,
        token_index: AtomicIsize,
        line: isize,
        column: isize,
        text: String,
        read_only: bool,
    ) -> Self {
        Self {
            token_type,
            channel,
            start,
            stop,
            token_index,
            line,
            column,
            text,
            read_only,
        }
    }
}

impl Display for BaseToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Token for BaseToken {
    #[inline]
    fn token_type(&self) -> isize {
        self.token_type
    }

    #[inline]
    fn text(&self) -> Cow<'_, str> {
        return if self.token_type == TOKEN_EOF {
            Cow::Borrowed(TEXT_EOF)
        } else {
            Cow::Borrowed(self.text.as_str())
        };
    }

    #[inline]
    fn token_index(&self) -> isize {
        self.token_index.load(Ordering::Relaxed)
    }

    #[inline]
    fn set_token_index(&self, idx: isize) {
        self.token_index.store(idx, Ordering::Relaxed)
    }
}

impl Clone for BaseToken {
    fn clone(&self) -> Self {
        Self {
            token_type: self.token_type,
            channel: self.channel,
            start: self.start,
            stop: self.start,
            token_index: AtomicIsize::new(self.token_index()),
            line: self.line,
            column: self.column,
            text: self.text.clone(),
            read_only: self.read_only,
        }
    }
}
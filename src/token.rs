use std::borrow::Cow;
use std::sync::atomic::{AtomicIsize, Ordering};
use crate::int_stream;

/// A token has properties: text, type, line, character position in the line
///  (so we can ignore tabs), token channel, index, and source from which
///  we obtained this token.
pub const TOKEN_INVALID_TYPE: isize = 0;

/// During lookahead operations, this "token" signifies we hit rule end ATN state
/// and did not follow it despite needing to.
pub const TOKEN_EPSILON: isize = -2;

pub const TOKEN_MIN_USER_TOKEN_TYPE: isize = 1;

pub const TOKEN_EOF: isize = int_stream::EOF;

/** All tokens go to the parser (unless skip() is called in that rule)
*  on a particular "channel".  The parser tunes to a particular channel
*  so that whitespace etc... can go to the parser on a "hidden" channel.
 */
pub const TOKEN_DEFAULT_CHANNEL: isize = 0;

/** Anything on different channel than DEFAULT_CHANNEL is not parsed
*  by parser.
 */
pub const TOKEN_HIDDEN_CHANNEL: isize = 1;

/**
 * This is the minimum constant value which can be assigned to a
 * user-defined token channel.
 *
 * <p>
 * The non-negative numbers less than {@link #MIN_USER_CHANNEL_VALUE} are
 * assigned to the predefined channels {@link #DEFAULT_CHANNEL} and
 * {@link #HIDDEN_CHANNEL}.</p>
 *
 * @see Token#getChannel()
 */
pub const TOKEN_MIN_USER_CHANNEL_VALUE: isize = 2;

pub trait Token<'a> {
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
    fn text(&'a self) -> Cow<'a, str>;

    /// An index from 0..n-1 of the token object in the input stream.
    /// This must be valid in order to print token streams and
    /// use TokenRewriteStream.
    /// Return -1 to indicate that this token was conjured up since
    /// it doesn't have a valid index.
    fn token_index(&self) -> isize { 0 }

    fn set_token_index(&self, idx: isize);
}

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

impl<'a> Token<'a> for BaseToken {
    #[inline]
    fn token_type(&self) -> isize {
        self.token_type
    }

    #[inline]
    fn text(&'a self) -> Cow<'a, str> {
        Cow::Borrowed(self.text.as_str())
    }

    #[inline]
    fn token_index(&self) -> isize {
        self.token_index.load(Ordering::SeqCst)
    }

    #[inline]
    fn set_token_index(&self, idx: isize) {
        self.token_index.store(idx, Ordering::SeqCst)
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
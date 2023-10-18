use std::rc::Rc;
use crate::char_stream::CharStream;
use crate::int_stream;
use crate::token_source::TokenSource;

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

pub trait Token {
    /// Get the text of the token.
    fn text(&self) -> String;

    /// Get the token type of the token */
    fn typ(&self) -> isize;

    /// The line number on which the 1st character of this token was matched,
    ///  line=1..n
    fn line(&self) -> isize;

    /// The index of the first character of this token relative to the
    ///  beginning of the line at which it occurs, 0..n-1
    fn char_position_in_line(&self) -> isize;

    /// Return the channel this token. Each token can arrive at the parser
    /// on a different channel, but the parser only "tunes" to a single channel.
    /// The parser ignores everything not on DEFAULT_CHANNEL.
    fn channel(&self) -> isize { TOKEN_DEFAULT_CHANNEL }

    /// An index from 0..n-1 of the token object in the input stream.
    /// This must be valid in order to print token streams and
    /// use TokenRewriteStream.
    /// Return -1 to indicate that this token was conjured up since
    /// it doesn't have a valid index.
    fn token_index(&self) -> isize;

    /// The starting character index of the token
    ///  This method is optional; return -1 if not implemented.
    fn start_index(&self) -> isize;

    /// The last character index of the token.
    ///  This method is optional; return -1 if not implemented.
    fn stop_index(&self) -> isize;

    // Gets the {@link TokenSource} which created this token.
    fn token_source(&self) -> Option<Rc<dyn TokenSource>>;

    // Gets the {@link CharStream} from which this token was derived.
    // fn input_stream(&self) -> Option<Rc<dyn CharStream>>;
}

pub struct BaseToken {
    pub token_type: isize,
    pub channel: isize,
    pub start: isize,
    pub stop: isize,
    pub token_index: isize,
    pub line: isize,
    pub column: isize,
    pub text: String,
    pub read_only: bool,
}
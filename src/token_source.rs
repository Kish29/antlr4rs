use crate::token::Token;

/// A source of tokens must provide a sequence of tokens via `next_token()`
//  and also must reveal it's source of characters; {@link CommonToken}'s text is
//  computed from a {@link CharStream}; it only store indices into the char
//  stream.
//
//  <p>Errors from the lexer are never passed to the parser. Either you want to keep
//  going or you do not upon token recognition error. If you do not want to
//  continue lexing then you do not want to continue parsing. Just throw an
//  exception not under {@link RecognitionException} and Java will naturally toss
//  you all the way out of the recognizers. If you want to continue lexing then
//  you should not throw an exception to the parser--it has already requested a
//  token. Keep lexing until you get a valid one. Just report errors and keep
//  going, looking for a valid token.</p>
pub trait TokenSource {
    type TK: Token + Clone + ?Sized;
    // type CS: CharStream;

    fn next_token(&mut self) -> Self::TK;

    fn line(&self) -> isize;

    fn char_position_in_line(&self) -> isize;

    // fn input_stream(&self) -> &Self::CS;
}
use std::fmt::Formatter;
use crate::char_stream::CharStream;
use crate::lexer_atn_simulator::LexerATNSimulator;
use crate::recognizer::Recognizer;
use crate::token::Token;
use crate::token_factory::TokenFactory;
use crate::token_source::TokenSource;

pub trait Lexer: TokenSource + Recognizer {
    type TK: Token + Clone + ?Sized;

    fn emit(&mut self) -> Self::TK;
}

pub struct BaseLexer<R, LAS, TF, CS>
    where R: Recognizer,
          LAS: LexerATNSimulator,
          TF: TokenFactory,
          CS: CharStream
{
    pub(crate) recognizer: R,
    pub(crate) interpreter: LAS,
    pub(crate) factory: TF,
    pub(crate) input: CS,
}

impl<R, LAS, TF, CS> BaseLexer<R, LAS, TF, CS>
    where R: Recognizer,
          LAS: LexerATNSimulator,
          TF: TokenFactory,
          CS: CharStream
{
    fn new(r: R, las: LAS, tf: TF, cs: CS) -> Self {
        Self {
            recognizer: r,
            interpreter: las,
            factory: tf,
            input: cs,
        }
    }
}

impl<R, LAS, TF, CS> TokenSource for BaseLexer<R, LAS, TF, CS>
    where CS: CharStream, LAS: LexerATNSimulator, R: Recognizer, TF: TokenFactory
{
    fn line(&self) -> isize {
        todo!()
    }

    fn char_position_in_line(&self) -> isize {
        todo!()
    }

    fn input_stream(&mut self) -> isize {
        todo!()
    }
}

impl<R, LAS, TF, CS> Recognizer for BaseLexer<R, LAS, TF, CS>
    where CS: CharStream, LAS: LexerATNSimulator, R: Recognizer, TF: TokenFactory
{
    fn literal_names(&self) -> &[&str] {
        self.recognizer.literal_names()
    }

    fn rule_names(&self) -> &[&str] {
        self.recognizer.rule_names()
    }
}

impl<R, LAS, TF, CS> Lexer for BaseLexer<R, LAS, TF, CS>
    where R: Recognizer,
          LAS: LexerATNSimulator,
          TF: TokenFactory,
          CS: CharStream
{
    type TK = TF::TK;

    fn emit(&mut self) -> Self::TK {
        todo!()
    }
}


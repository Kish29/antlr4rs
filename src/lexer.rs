use std::cell::RefCell;
use std::rc::Rc;
use crate::error_listener::ErrorListener;
use crate::atn::ATN;
use crate::char_stream::CharStream;
use crate::errors::ANTLRError;
use crate::lexer_atn_simulator::LexerATNSimulator;
use crate::recognizer::Recognizer;
use crate::token::{Token, TOKEN_DEFAULT_CHANNEL, TOKEN_INVALID_TYPE};
use crate::token_factory::TokenFactory;
use crate::token_source::TokenSource;

pub const LEXER_DEFAULT_MODE: isize = 0;
pub const LEXER_MORE: isize = -2;
pub const LEXER_SKIP: isize = -3;

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

    token_start_idx: isize,
    token_start_line: isize,
    token_start_column: isize,

    // token that temporary store and to emit
    token: Option<TF::TK>,
    hit_eof: bool,
    channel: isize,
    this_type: isize,
    mode_stack: Vec<isize>,
    mode: isize,
    text: Option<String>,
}

impl<R, LAS, TF, CS> BaseLexer<R, LAS, TF, CS>
    where R: Recognizer,
          LAS: LexerATNSimulator,
          TF: TokenFactory,
          CS: CharStream
{
    pub fn new(r: R, las: LAS, tf: TF, cs: CS) -> Self {
        Self {
            recognizer: r,
            interpreter: las,
            factory: tf,
            input: cs,
            token_start_idx: -1,
            token_start_line: -1,
            token_start_column: -1,
            token: None,
            hit_eof: false,
            channel: TOKEN_DEFAULT_CHANNEL,
            this_type: TOKEN_INVALID_TYPE,
            mode_stack: vec![],
            mode: LEXER_DEFAULT_MODE,
            text: None,
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

    fn atn(&self) -> &ATN {
        self.recognizer.atn()
    }

    fn state(&self) -> isize {
        self.recognizer.state()
    }

    fn set_state(&mut self, state: isize) {
        self.recognizer.set_state(state)
    }

    fn add_error_listener(&mut self, l: Rc<RefCell<dyn ErrorListener>>) {
        self.recognizer.add_error_listener(l)
    }

    fn remove_error_listeners(&mut self) {
        self.recognizer.remove_error_listeners()
    }

    fn error_listener_dispatch(&self) -> Rc<RefCell<dyn ErrorListener>> {
        self.recognizer.error_listener_dispatch()
    }

    fn has_error(&self) -> bool {
        self.recognizer.has_error()
    }

    fn error(&self) -> Option<&ANTLRError> {
        self.recognizer.error()
    }

    fn set_error(&mut self, e: ANTLRError) {
        self.recognizer.set_error(e)
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


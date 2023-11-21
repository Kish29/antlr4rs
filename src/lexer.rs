use std::cell::RefCell;
use std::rc::Rc;
use crate::error_listener::ErrorListener;
use crate::char_stream::CharStream;
use crate::errors::ANTLRError;
use crate::lexer_atn_simulator::LexerATNSimulator;
use crate::recognizer::Recognizer;
use crate::rule_context::RuleContext;
use crate::token::{TOKEN_DEFAULT_CHANNEL, TOKEN_EOF, TOKEN_INVALID_TYPE};
use crate::token_factory::TokenFactory;
use crate::token_source::TokenSource;

pub const LEXER_DEFAULT_MODE: isize = 0;
pub const LEXER_MORE: isize = -2;
pub const LEXER_SKIP: isize = -3;

pub trait Lexer: TokenSource + Recognizer {
    fn emit(&mut self) -> Self::TK;
}

pub struct BaseLexer<R, LAS, TF, CS>
    where R: Recognizer,
          LAS: LexerATNSimulator,
          TF: TokenFactory,
          CS: CharStream
{
    // recognizer for specified lexer literals and rules
    pub(crate) recognizer: R,
    // ATN simulator for lexer
    pub(crate) interpreter: LAS,
    // token factory for creating token
    pub(crate) factory: TF,
    // the source of char stream to create token
    pub(crate) input: CS,

    // store the current token start position in line
    token_start_idx: isize,
    // store the current token start position of line
    token_start_line: isize,
    // store the current token start position of column
    token_start_column: isize,

    // token that temporary store and to emit
    token: Option<TF::TK>,
    // indicate that stream whether hit eof
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

impl<R, LAS, TF, CS> BaseLexer<R, LAS, TF, CS>
    where CS: CharStream, LAS: LexerATNSimulator, R: Recognizer, TF: TokenFactory {
    pub fn emit_token(&mut self, tk: TF::TK) {
        self.token = Some(tk)
    }

    pub fn emit_eof(&mut self) {
        let eof = self.factory.create(
            &self.input,
            TOKEN_EOF,
            self.text.take(),
            TOKEN_DEFAULT_CHANNEL,
            self.input.index(),
            self.input.index() - 1,
            self.line(),
            self.char_position_in_line(),
        );
        self.emit_token(eof)
    }

    pub fn emit(&mut self) -> TF::TK {
        todo!()
    }
}

impl<R, LAS, TF, CS> TokenSource for BaseLexer<R, LAS, TF, CS>
    where CS: CharStream, LAS: LexerATNSimulator, R: Recognizer, TF: TokenFactory
{
    type TK = TF::TK;
    // type CS = CS;

    // parse the next token
    fn next_token(&mut self) -> Self::TK {
        'outer: loop {
            // if stream hit the eof
            if self.hit_eof {
                // set and return eof token
                self.emit_eof();
                break;
            }
            // reset all status
            self.token = None;
            self.channel = TOKEN_DEFAULT_CHANNEL;
            // token start position: column index
            self.token_start_column = self.interpreter.char_position_in_line();
            // line index
            self.token_start_line = self.line();
            self.text = None;

            let idx = self.input.index();
            'inner: loop {
                // self.interpreter.match_()
            }
        }
        self.token.take().unwrap()
    }

    fn line(&self) -> isize {
        self.interpreter.line()
    }

    fn char_position_in_line(&self) -> isize {
        self.interpreter.char_position_in_line()
    }

    // fn input_stream(&self) -> &Self::CS {
    //     &self.input
    // }
}

impl<R, LAS, TF, CS> Recognizer for BaseLexer<R, LAS, TF, CS>
    where CS: CharStream, LAS: LexerATNSimulator, R: Recognizer, TF: TokenFactory
{
    fn literal_names(&self) -> &[&str] {
        self.recognizer.literal_names()
    }

    fn symbolic_names(&self) -> &[&str] {
        self.recognizer.symbolic_names()
    }

    fn rule_names(&self) -> &[&str] {
        self.recognizer.rule_names()
    }

    fn sempred(&self, _local_ctx: Rc<dyn RuleContext>, _rule_idx: isize, _action_idx: isize) -> bool {
        todo!()
    }

    fn precpred(&self, _local_ctx: Rc<dyn RuleContext>, _precedence: isize) -> bool {
        todo!()
    }

    /*fn atn(&self) -> &ATN {
        self.recognizer.atn()
    }*/

    fn state(&self) -> isize {
        self.recognizer.state()
    }

    fn set_state(&mut self, state: isize) {
        self.recognizer.set_state(state)
    }

    fn action(&self, _local_ctx: Rc<dyn RuleContext>, _rule_idx: isize, _action_idx: isize) {
        todo!()
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
    fn emit(&mut self) -> Self::TK {
        todo!()
    }
}


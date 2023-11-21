use crate::atn::ATN;
use crate::atn_simulator::{ATNSimulator, BaseATNSimulator};
use crate::char_stream::CharStream;
use crate::dfa::DFA;
use crate::lexer::LEXER_DEFAULT_MODE;
use crate::prediction_context::PredictionContextCache;

pub trait LexerATNSimulator: ATNSimulator {
    fn reset(&mut self);

    fn match_(&mut self, input: &mut dyn CharStream, mode: isize) -> isize;

    fn char_position_in_line(&self) -> isize;

    fn line(&self) -> isize;

    fn consume(&self, input: &mut dyn CharStream);
}

#[derive(Debug)]
pub(crate) struct SimState {
    index: isize,
    line: isize,
    column: isize,
    dfa_state: Option<usize>,
}

impl SimState {
    pub(crate) fn new() -> Self {
        Self {
            index: -1,
            line: 0,
            column: 0,
            dfa_state: None,
        }
    }

    pub(crate) fn reset(&mut self) {
        self.dfa_state = None
    }
}

pub struct BaseLexerATNSimulator {
    base: BaseATNSimulator,
    pub(crate) mode: isize,
    prev_accept: SimState,
}

impl BaseLexerATNSimulator {
    pub fn new(atn: ATN, shared_ctx_cache: PredictionContextCache) -> Self {
        Self {
            base: BaseATNSimulator::new(atn, shared_ctx_cache),
            mode: LEXER_DEFAULT_MODE,
            prev_accept: SimState::new(),
        }
    }
}

impl ATNSimulator for BaseLexerATNSimulator {
    #[inline(always)]
    fn shared_context_cache(&self) -> &PredictionContextCache {
        self.base.shared_context_cache()
    }

    fn atn(&self) -> &ATN {
        todo!()
    }

    fn decision_to_dfa(&self) -> &Vec<DFA> {
        todo!()
    }
}

impl LexerATNSimulator for BaseLexerATNSimulator {
    fn reset(&mut self) {
        todo!()
    }

    fn match_(&mut self, input: &mut dyn CharStream, mode: isize) -> isize {
        self.mode = mode;

        input.text(1, 2);
        todo!()
    }

    fn char_position_in_line(&self) -> isize {
        todo!()
    }

    fn line(&self) -> isize {
        todo!()
    }

    fn consume(&self, input: &mut dyn CharStream) {
        todo!()
    }
}
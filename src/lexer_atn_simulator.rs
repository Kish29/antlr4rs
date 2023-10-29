use crate::atn::ATN;
use crate::atn_simulator::ATNSimulator;
use crate::char_stream::CharStream;
use crate::dfa::DFA;
use crate::prediction_context::PredictionContextCache;

pub trait LexerATNSimulator: ATNSimulator {
    fn reset(&mut self);

    fn match_(&mut self, input: &mut dyn CharStream);

    fn char_position_in_line(&self) -> isize;

    fn line(&self) -> isize;

    fn consume(&self, input: &mut dyn CharStream);
}

pub struct BaseLexerATNSimulator {}

impl BaseLexerATNSimulator {
    pub fn new() -> Self {
        Self {}
    }
}

impl ATNSimulator for BaseLexerATNSimulator {
    fn shared_context_cache(&self) -> &PredictionContextCache {
        todo!()
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

    fn match_(&mut self, input: &mut dyn CharStream) {
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
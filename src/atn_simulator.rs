use crate::atn::ATN;
use crate::dfa::DFA;
use crate::prediction_context::PredictionContextCache;

pub trait ATNSimulator<'a> {
    fn shared_context_cache(&self) -> &PredictionContextCache;

    fn atn(&self) -> &ATN;

    fn decision_to_dfa(&self) -> &Vec<DFA>;
}
use crate::atn::ATN;
use crate::dfa::DFA;
use crate::prediction_context::PredictionContextCache;

pub trait ATNSimulator {
    fn shared_context_cache(&self) -> &PredictionContextCache;

    fn atn(&self) -> &ATN;

    fn decision_to_dfa(&self) -> &Vec<DFA>;
}


#[derive(Debug)]
pub struct BaseATNSimulator {
    atn: ATN,
    shared_ctx_cache: PredictionContextCache,
    decision_to_dfa: Vec<DFA>,
}

impl BaseATNSimulator {
    pub fn new(atn: ATN, shared_ctx_cache: PredictionContextCache) -> Self {
        Self {
            atn,
            shared_ctx_cache,
            decision_to_dfa: vec![],
        }
    }
}

impl ATNSimulator for BaseATNSimulator {
    #[inline(always)]
    fn shared_context_cache(&self) -> &PredictionContextCache {
        &self.shared_ctx_cache
    }

    #[inline(always)]
    fn atn(&self) -> &ATN {
        &self.atn
    }

    #[inline(always)]
    fn decision_to_dfa(&self) -> &Vec<DFA> {
        &self.decision_to_dfa
    }
}
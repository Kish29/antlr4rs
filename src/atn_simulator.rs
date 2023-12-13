use std::sync::{Arc, RwLock};
use crate::atn::ATN;
use crate::dfa::DFA;
use crate::prediction_context::PredictionContextCache;

pub trait ATNSimulator {
    fn shared_context_cache(&self) -> &RwLock<PredictionContextCache>;

    fn atn(&self) -> &ATN;

    fn decision_to_dfa(&self) -> &Vec<RwLock<DFA>>;
}


#[derive(Debug)]
pub struct BaseATNSimulator {
    atn: Arc<ATN>,
    shared_ctx_cache: Arc<RwLock<PredictionContextCache>>,
    decision_to_dfa: Arc<Vec<RwLock<DFA>>>,
}

impl BaseATNSimulator {
    // #[inline(always)]
    pub fn new(
        atn: Arc<ATN>,
        shared_ctx_cache: Arc<RwLock<PredictionContextCache>>,
        decision_to_dfa: Arc<Vec<RwLock<DFA>>>,
    ) -> Self {
        Self {
            atn,
            shared_ctx_cache,
            decision_to_dfa,
        }
    }
}

impl ATNSimulator for BaseATNSimulator {
    // #[inline(always)]
    fn shared_context_cache(&self) -> &RwLock<PredictionContextCache> {
        &self.shared_ctx_cache
    }

    // #[inline(always)]
    fn atn(&self) -> &ATN {
        &self.atn
    }

    // #[inline(always)]
    fn decision_to_dfa(&self) -> &Vec<RwLock<DFA>> {
        &self.decision_to_dfa
    }
}
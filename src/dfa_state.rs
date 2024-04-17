use std::fmt::{Display, Formatter};
use crate::atn::ATN_INVALID_ALT_NUMBER;
use crate::atn_config_set::ATNConfigSet;
use crate::misc::murmur3::MurmurHash;
use crate::Nth;
use crate::semantic_context::SemanticContext;

#[derive(Debug)]
pub struct DFAState {
    /// nth(order/position) of this state in [crate::dfa::DFA]'s states.
    pub(crate) state_nth: Nth,
    pub(crate) configs: Box<ATNConfigSet>,
    /// store edges(nth/order/position of other [DFAState]) in [crate::dfa::DFA]'s states.
    pub edges: Vec<Nth>,
    pub accept_state: bool,
    /// [prediction] is the ttype we match or alt we predict if the state is accept.
    /// Set to [ATN_INVALID_ALT_NUMBER] when [predicates] is not empty or [requires_full_context] is true.
    pub prediction: isize,
    pub requires_full_context: bool,
    pub predicates: Vec<PredPrediction>,
}

/// [PredPrediction] maps a predicate to a predicted alternative.
#[derive(Debug)]
pub struct PredPrediction {
    // never null; at least SemanticContext.NONE
    pub pred: SemanticContext,
    pub alt: isize,
}

impl Display for PredPrediction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.pred, self.alt))
    }
}

impl PartialEq for DFAState {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl MurmurHash for DFAState {
    // #[inline]
    fn murmur(&self) -> u32 {
        todo!()
    }
}

impl DFAState {
    // #[inline(always)]
    pub fn new(state_nth: Nth, configs: Box<ATNConfigSet>) -> Self {
        Self {
            state_nth,
            configs,
            edges: vec![],
            accept_state: false,
            prediction: ATN_INVALID_ALT_NUMBER,
            requires_full_context: false,
            predicates: vec![],
        }
    }
}
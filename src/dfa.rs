use std::collections::HashMap;
use std::sync::Arc;
use crate::atn::{ATN, ATN_INVALID_ALT_NUMBER};
use crate::atn_config_set::ATNConfigSet;
use crate::atn_state::ATNState;
use crate::dfa_state::DFAState;
use crate::misc::jcollect::JStore;
use crate::misc::murmur3::MurmurHash;
use crate::Nth;

#[derive(Debug)]
pub struct DFA {
    ///  the ATN state position from ATN, which this DFA was started from.
    pub atn_start_state: Nth,
    pub decision: usize,

    states: JStore<DFAState>,

    // state 0
    pub s0: Option<DFAState>,

    pub(crate) precedence_dfa: bool,
}

impl DFA {
    // #[inline]
    pub fn new(atn: &ATN, atn_start_state: usize, decision: usize) -> Self {
        let mut dfa = Self {
            atn_start_state,
            decision,
            s0: None,
            precedence_dfa: false,
            states: JStore::default(),
        };
        if let ATNState::StarLoopEntry(sle) = &atn.states[atn_start_state] {
            if sle.precedence_decision {
                dfa.precedence_dfa = true;
                let mut ds = DFAState::new(0, Box::new(ATNConfigSet::new(false)));
                ds.accept_state = false;
                ds.requires_full_context = false;
                dfa.s0 = Some(ds);
            }
        }
        dfa
    }
}
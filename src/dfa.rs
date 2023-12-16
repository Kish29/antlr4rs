use std::collections::HashMap;
use crate::atn::ATN;
use crate::atn_state::ATNState;
use crate::dfa_state::DFAState;
use crate::Nth;

#[derive(Debug)]
pub struct DFA {
    // decision to atn state position
    pub dcs2state_nth: Nth,
    pub decision: usize,

    states: Vec<DFAState>,
    fast_states: HashMap</*hash(DFAState)*/u128, Vec<Nth>>,

    // state 0
    pub s0: Option<Nth>,

    pub precedence_dfa: bool,
}

impl DFA {
    // #[inline]
    pub fn new(atn: &ATN, dcs2state_nth: usize, decision: usize) -> Self {
        let mut dfa = Self {
            dcs2state_nth,
            decision,
            s0: None,
            precedence_dfa: false,
            states: Vec::with_capacity(16),
            fast_states: HashMap::with_capacity(16),
        };
        if let ATNState::StarLoopEntry(sle) = &atn.states[dcs2state_nth] {
            if sle.precedence_decision {
                dfa.precedence_dfa = true;
                todo!()
            }
        }
        dfa
    }
}

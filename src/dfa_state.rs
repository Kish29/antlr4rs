use std::rc::Rc;
use crate::atn_config_set::ATNConfigSet;
use crate::Nth;

#[derive(Debug)]
pub struct DFAState {
    /// nth(order/position) of this state in [crate::dfa::DFA]'s states.
    pub(crate) state_nth: Nth,
    pub(crate) configs: Rc<ATNConfigSet>,
    /// store edges(nth/order/position of other [DFAState]) in [crate::dfa::DFA]'s states.
    pub edges: Vec<Nth>,
    pub accept_state: bool,
    // the number of input prediction
    pub prediction: isize,

}

impl DFAState {
    // #[inline(always)]
    pub fn new() -> Self {
        todo!()
    }
}
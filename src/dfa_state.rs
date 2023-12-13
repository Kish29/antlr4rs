use std::rc::Rc;
use crate::atn_config_set::ATNConfigSet;

pub struct DFAState {
    pub(crate) state_number: usize,
    pub(crate) configs: Rc<ATNConfigSet>,

}

impl DFAState {
    // #[inline(always)]
    pub fn new() -> Self {
        todo!()
    }
}
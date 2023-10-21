use crate::atn_config_set::ATNConfigSet;

pub struct DFAState {

    pub(crate) state_number: usize,
    configs: Box<ATNConfigSet>,

}
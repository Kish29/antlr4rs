use std::collections::HashMap;
use crate::atn_state::{ATNState, ATNStateType};
use crate::atn_type::ATNType;
use crate::lexer_action::LexerAction;

pub(crate) const ATN_INVALID_ALT_NUMBER: isize = 0;

#[derive(Debug)]
pub struct ATN {
    pub(crate) grammar_type: ATNType,
    pub(crate) decision2state_nth: Vec<usize>,
    pub(crate) lexer_actions: Vec<LexerAction>,
    max_token_type: isize,
    mode_name_to_start_state: HashMap<String, ATNStateType>,
    pub(crate) mode2start_state_nths: Vec<usize>,
    pub(crate) rule2start_state_nths: Vec<usize>,
    pub(crate) rule2stop_state_nths: Vec<usize>,
    pub(crate) rule2token_type: Vec<usize>,

    pub(crate) states: Vec<ATNState>,
}

impl ATN {
    // #[inline(always)]
    pub fn new(grammar_type: ATNType, max_token_type: isize) -> Self {
        Self {
            grammar_type,
            decision2state_nth: vec![],
            lexer_actions: vec![],
            max_token_type,
            mode_name_to_start_state: HashMap::new(),
            mode2start_state_nths: vec![],
            rule2start_state_nths: vec![],
            rule2stop_state_nths: vec![],
            rule2token_type: vec![],
            states: vec![],
        }
    }
}
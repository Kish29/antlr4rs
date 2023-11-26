use std::collections::HashMap;
use crate::atn_state::{ATNState, BaseATNState, StateType};
use crate::atn_type::ATNType;
use crate::lexer_action::LexerAction;

pub(crate) const ATN_INVALID_ALT_NUMBER: isize = 0;

#[derive(Debug)]
pub struct ATN {
    grammar_type: ATNType,
    decision_to_state: Vec<StateType>,
    lexer_actions: Vec<LexerAction>,
    max_token_type: isize,
    mode_name_to_start_state: HashMap<String, StateType>,
    mode_to_start_state: Vec<StateType>,
    rule_to_start_state: Vec<StateType>,
    rule_to_stop_state: Vec<StateType>,
    rule_to_token_type: Vec<isize>,

    pub(crate) states: Vec<ATNState>,
}

impl ATN {
    #[inline(always)]
    pub fn new(grammar_type: ATNType, max_token_type: isize) -> Self {
        Self {
            grammar_type,
            decision_to_state: vec![],
            lexer_actions: vec![],
            max_token_type,
            mode_name_to_start_state: HashMap::new(),
            mode_to_start_state: vec![],
            rule_to_start_state: vec![],
            rule_to_stop_state: vec![],
            rule_to_token_type: vec![],
            states: vec![],
        }
    }
}
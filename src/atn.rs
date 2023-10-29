use std::collections::HashMap;
use crate::atn_state::ATNState;
use crate::atn_type::ATNType;
use crate::lexer_action::LexerAction;

pub struct ATN {
    pub(crate) grammar_type: ATNType,
    pub(crate) decision_to_state: Vec<ATNState>,
    pub(crate) lexer_actions: Vec<LexerAction>,
    pub(crate) max_token_type: isize,
    pub(crate) mode_name_to_start_state: HashMap<String, ATNState>,
    pub(crate) mode_to_start_state: Vec<ATNState>,
    pub(crate) rule_to_start_state: Vec<ATNState>,
    pub(crate) rule_to_stop_state: Vec<ATNState>,
    pub(crate) rule_to_token_type: Vec<isize>,

    pub(crate) states: Vec<ATNState>,
}

impl ATN {
    pub(crate) fn new(grammar_type: ATNType, max_token_type: isize) -> Self {
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
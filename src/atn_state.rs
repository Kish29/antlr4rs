pub(crate) type StateType = usize;

pub(crate) const ATN_STATE_BASIC: StateType = 1;
// expr : expr
pub(crate) const ATN_STATE_RULE_START: StateType = 2;
// (xxx)
pub(crate) const ATN_STATE_BLOCK_START: StateType = 3;
// (xxx)+
pub(crate) const ATN_STATE_PLUS_BLOCK_START: StateType = 4;
// (xxx)*
pub(crate) const ATN_STATE_STAR_BLOCK_START: StateType = 5;
pub(crate) const ATN_STATE_TOKEN_START: StateType = 6;
pub(crate) const ATN_STATE_RULE_STOP: StateType = 7;
pub(crate) const ATN_STATE_BLOCK_END: StateType = 8;
pub(crate) const ATN_STATE_STAR_LOOPBACK: StateType = 9;
pub(crate) const ATN_STATE_STAR_LOOP_ENTRY: StateType = 10;
pub(crate) const ATN_STATE_PLUS_LOOPBACK: StateType = 11;
pub(crate) const ATN_STATE_LOOP_END: StateType = 12;

#[derive(Debug)]
pub struct BaseATNState {
    state_nth: usize,
    state_type: StateType,
    rule_idx: usize,
}

impl BaseATNState {
    #[inline(always)]
    pub fn new(state_type: StateType, rule_idx: usize, state_nth: usize) -> Self {
        Self { state_type, rule_idx, state_nth }
    }
}

#[derive(Debug)]
pub enum ATNState {
    Basic(BaseATNState),
    RuleStart(RuleStartState),
    BlockStart(BlockStartState),
    PlusBlockStart(PlusBlockStartState),
    StarBlockStart(StarBlockStartState),
    TokenStart(TokenStartState),
    RuleStop(RuleStopState),
    BlockEnd(BlockEndState),
    StarLoopback(StarLoopbackState),
    StarLoopEntry(StarLoopEntryState),
    PlusLoopback(PlusLoopbackState),
    LoopEnd(LoopEndState),
}

impl ATNState {
    #[inline]
    pub fn new(state_type: StateType, rule_idx: usize, state_nth: usize) -> ATNState {
        match state_type {
            ATN_STATE_BASIC => ATNState::new_basic(rule_idx, state_nth),
            ATN_STATE_RULE_START => ATNState::new_rule_start(rule_idx, state_nth),
            ATN_STATE_BLOCK_START => ATNState::new_block_start(rule_idx, state_nth),
            ATN_STATE_PLUS_BLOCK_START => ATNState::new_plus_block_start(rule_idx, state_nth),
            ATN_STATE_STAR_BLOCK_START => ATNState::new_star_block_start(rule_idx, state_nth),
            ATN_STATE_TOKEN_START => ATNState::new_token_start(rule_idx, state_nth),
            ATN_STATE_RULE_STOP => ATNState::new_rule_stop(rule_idx, state_nth),
            ATN_STATE_BLOCK_END => ATNState::new_block_end(rule_idx, state_nth),
            ATN_STATE_STAR_LOOPBACK => ATNState::new_star_loopback(rule_idx, state_nth),
            ATN_STATE_STAR_LOOP_ENTRY => ATNState::new_star_loop_entry(rule_idx, state_nth),
            ATN_STATE_PLUS_LOOPBACK => ATNState::new_plus_loopback(rule_idx, state_nth),
            ATN_STATE_LOOP_END => ATNState::new_loop_end(rule_idx, state_nth),
            _ => panic!("state type {} is invalid", state_type)
        }
    }

    #[inline(always)]
    fn new_basic(rule_idx: usize, state_nth: usize) -> ATNState {
        ATNState::Basic(BaseATNState::new(ATN_STATE_BASIC, rule_idx, state_nth))
    }

    #[inline(always)]
    fn new_rule_start(rule_idx: usize, state_nth: usize) -> ATNState {
        todo!()
    }

    #[inline(always)]
    fn new_block_start(rule_idx: usize, state_nth: usize) -> ATNState {
        todo!()
    }
    #[inline(always)]
    fn new_plus_block_start(rule_idx: usize, state_nth: usize) -> ATNState {
        todo!()
    }
    #[inline(always)]
    fn new_star_block_start(rule_idx: usize, state_nth: usize) -> ATNState {
        todo!()
    }
    #[inline(always)]
    fn new_token_start(rule_idx: usize, state_nth: usize) -> ATNState {
        todo!()
    }
    #[inline(always)]
    fn new_rule_stop(rule_idx: usize, state_nth: usize) -> ATNState {
        todo!()
    }
    #[inline(always)]
    fn new_block_end(rule_idx: usize, state_nth: usize) -> ATNState {
        todo!()
    }
    #[inline(always)]
    fn new_star_loopback(rule_idx: usize, state_nth: usize) -> ATNState {
        todo!()
    }
    #[inline(always)]
    fn new_star_loop_entry(rule_idx: usize, state_nth: usize) -> ATNState {
        todo!()
    }
    #[inline(always)]
    fn new_plus_loopback(rule_idx: usize, state_nth: usize) -> ATNState {
        todo!()
    }
    #[inline(always)]
    fn new_loop_end(rule_idx: usize, state_nth: usize) -> ATNState {
        todo!()
    }

    #[inline(always)]
    pub fn state_type(&self) -> StateType {
        match self {
            ATNState::Basic(b) => b.state_type,
            ATNState::RuleStart(r) => {}
            ATNState::BlockStart(_) => {}
            ATNState::PlusBlockStart(_) => {}
            ATNState::StarBlockStart(_) => {}
            ATNState::TokenStart(_) => {}
            ATNState::RuleStop(_) => {}
            ATNState::BlockEnd(_) => {}
            ATNState::StarLoopback(_) => {}
            ATNState::StarLoopEntry(_) => {}
            ATNState::PlusLoopback(_) => {}
            ATNState::LoopEnd(_) => {}
        }
    }
}

#[derive(Debug)]
pub struct DecisionState {
    pub(crate) base: BaseATNState,
    pub(crate) decision: isize,
    pub(crate) non_greedy: bool,
}

impl DecisionState {
    #[inline(always)]
    pub fn new(state_type: StateType, rule_idx: usize, decision: isize) -> Self {
        Self { base: BaseATNState::new(state_type, rule_idx), decision, non_greedy: false }
    }
}

#[derive(Debug)]
pub struct BlockStartState {
    pub base: BaseATNState,

}

#[derive(Debug)]
pub struct BlockEndState {}

#[derive(Debug)]
pub struct RuleStartState {}

#[derive(Debug)]
pub struct RuleStopState {}

#[derive(Debug)]
pub struct PlusBlockStartState {}

#[derive(Debug)]
pub struct StarBlockStartState {}

#[derive(Debug)]
pub struct TokenStartState {}

#[derive(Debug)]
pub struct StarLoopbackState {}

#[derive(Debug)]
pub struct StarLoopEntryState {}

#[derive(Debug)]
pub struct PlusLoopbackState {}

#[derive(Debug)]
pub struct LoopEndState {}
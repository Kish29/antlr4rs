pub(crate) type StateType = usize;

const ATN_STATE_BASIC: StateType = 1;

/// [ATN_STATE_RULE_START]/[RuleStartState] is the start of todo
const ATN_STATE_RULE_START: StateType = 2;

/// [ATN_STATE_BLOCK_START]/[BlockStartState] is the start of a regular (...) block.
const ATN_STATE_BLOCK_START: StateType = 3;

/// [ATN_STATE_PLUS_BLOCK_START]/[PlusBlockStartState] is the start of a (A|B|...)+ loop. Technically it is a
/// decision state; we don't use it for code generation. Somebody might need it,
/// it is included for completeness. In reality, [ATN_STATE_PLUS_BLOCK_START]/[PlusBlockStartState] is the real
/// decision-making node for A+.
const ATN_STATE_PLUS_BLOCK_START: StateType = 4;

/// [StarBlockStartState] is the block that begins a closure (A|B|...)* loop.
const ATN_STATE_STAR_BLOCK_START: StateType = 5;

/// [ATN_STATE_TOKEN_START]/[TokenStartState] is the Tokens rule start state linking to each lexer rule start state.
const ATN_STATE_TOKEN_START: StateType = 6;

/// [ATN_STATE_RULE_STOP]/[RuleStopState] is the last node in the ATN for a rule, unless that rule is the
/// start symbol. In that case, there is one transition to EOF. Later, we might
/// encode references to all calls to this rule to compute FOLLOW sets for error handling.
const ATN_STATE_RULE_STOP: StateType = 7;

/// [ATN_STATE_BLOCK_END]/[BlockEndState] is a terminal node of a simple (a|b|c) block.
const ATN_STATE_BLOCK_END: StateType = 8;

/// [ATN_STATE_STAR_LOOPBACK]/[StarLoopbackState] is todo
const ATN_STATE_STAR_LOOPBACK: StateType = 9;

/// [ATN_STATE_STAR_LOOP_ENTRY]/[StarLoopEntryState] is todo
const ATN_STATE_STAR_LOOP_ENTRY: StateType = 10;

/// [ATN_STATE_PLUS_LOOPBACK]/[PlusLoopbackState] is a decision state for A+ and (A|B)+. It has two
/// transitions: one to the loop back to start of the block, and one to exit.
const ATN_STATE_PLUS_LOOPBACK: StateType = 11;

/// [ATN_STATE_LOOP_END]/[LoopEndState] marks the end of a * or + loop.
const ATN_STATE_LOOP_END: StateType = 12;

#[derive(Debug)]
pub struct BaseATNState {
    state_nth: usize,
    state_type: StateType,
    pub(crate) rule_idx: usize,
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
    #[inline(always)]
    pub fn check_type(state_type: StateType) {
        if state_type < ATN_STATE_BASIC || state_type > ATN_STATE_LOOP_END {
            panic!("deserialize an invalid ATN state type, it should not happen, it's a bug.")
        }
    }

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
        let base = BaseATNState::new(ATN_STATE_RULE_START, rule_idx, state_nth);
        ATNState::RuleStart(RuleStartState { base, rule_stop_state_nth: 0, contrast_set: false, precedence: false })
    }

    #[inline(always)]
    fn new_block_start(rule_idx: usize, state_nth: usize) -> ATNState {
        let base = DecisionState::new(ATN_STATE_BLOCK_START, rule_idx, state_nth);
        ATNState::BlockStart(BlockStartState { base, block_end_state_nth: 0, contrast_set: false })
    }

    #[inline(always)]
    fn new_plus_block_start(rule_idx: usize, state_nth: usize) -> ATNState {
        let base = BlockStartState::new(ATN_STATE_PLUS_BLOCK_START, rule_idx, state_nth);
        ATNState::PlusBlockStart(PlusBlockStartState { base, plus_loopback_state_nth: 0, contrast_set: false })
    }

    #[inline(always)]
    fn new_star_block_start(rule_idx: usize, state_nth: usize) -> ATNState {
        let base = BlockStartState::new(ATN_STATE_STAR_BLOCK_START, rule_idx, state_nth);
        ATNState::StarBlockStart(StarBlockStartState { base })
    }

    #[inline(always)]
    fn new_token_start(rule_idx: usize, state_nth: usize) -> ATNState {
        let base = DecisionState::new(ATN_STATE_TOKEN_START, rule_idx, state_nth);
        ATNState::TokenStart(TokenStartState { base })
    }

    #[inline(always)]
    fn new_rule_stop(rule_idx: usize, state_nth: usize) -> ATNState {
        let base = BaseATNState::new(ATN_STATE_RULE_STOP, rule_idx, state_nth);
        ATNState::RuleStop(RuleStopState { base })
    }

    #[inline(always)]
    fn new_block_end(rule_idx: usize, state_nth: usize) -> ATNState {
        let base = BaseATNState::new(ATN_STATE_BLOCK_END, rule_idx, state_nth);
        ATNState::BlockEnd(BlockEndState { base, block_start_state_nth: 0, contrast_set: false })
    }

    #[inline(always)]
    fn new_star_loopback(rule_idx: usize, state_nth: usize) -> ATNState {
        let base = BaseATNState::new(ATN_STATE_STAR_LOOPBACK, rule_idx, state_nth);
        ATNState::StarLoopback(StarLoopbackState { base })
    }

    #[inline(always)]
    fn new_star_loop_entry(rule_idx: usize, state_nth: usize) -> ATNState {
        let base = DecisionState::new(ATN_STATE_STAR_LOOP_ENTRY, rule_idx, state_nth);
        ATNState::StarLoopEntry(StarLoopEntryState { base, star_loopback_state_nth: 0, contrast_set: false })
    }

    #[inline(always)]
    fn new_plus_loopback(rule_idx: usize, state_nth: usize) -> ATNState {
        let base = DecisionState::new(ATN_STATE_PLUS_LOOPBACK, rule_idx, state_nth);
        ATNState::PlusLoopback(PlusLoopbackState { base })
    }

    #[inline(always)]
    fn new_loop_end(rule_idx: usize, state_nth: usize) -> ATNState {
        let base = BaseATNState::new(ATN_STATE_LOOP_END, rule_idx, state_nth);
        ATNState::LoopEnd(LoopEndState { base, loopback_state_nth: 0, contrast_set: false })
    }

    #[inline(always)]
    pub fn to_decision_state_mut(&mut self) -> Option<&mut DecisionState> {
        match self {
            ATNState::BlockStart(b) => Some(&mut b.base),
            ATNState::PlusBlockStart(p) => Some(&mut p.base.base),
            ATNState::StarBlockStart(s) => Some(&mut s.base.base),
            ATNState::TokenStart(t) => Some(&mut t.base),
            ATNState::StarLoopEntry(s) => Some(&mut s.base),
            ATNState::PlusLoopback(p) => Some(&mut p.base),
            _ => None,
        }
    }

    #[inline(always)]
    pub fn to_rule_start_state_mut(&mut self) -> Option<&mut RuleStartState> {
        match self {
            ATNState::RuleStart(r) => Some(r),
            _ => None,
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
    fn new(state_type: StateType, rule_idx: usize, state_nth: usize) -> Self {
        let base = BaseATNState::new(state_type, rule_idx, state_nth);
        Self { base, decision: -1, non_greedy: false }
    }
}

#[derive(Debug)]
pub struct BlockStartState {
    pub(crate) base: DecisionState,
    pub(crate) block_end_state_nth: usize,
    pub(crate) contrast_set: bool,
}

impl BlockStartState {
    #[inline(always)]
    fn new(state_type: StateType, rule_idx: usize, state_nth: usize) -> Self {
        let base = DecisionState::new(state_type, rule_idx, state_nth);
        Self { base, block_end_state_nth: 0, contrast_set: false }
    }
}

#[derive(Debug)]
pub struct BlockEndState {
    pub(crate) base: BaseATNState,
    pub(crate) block_start_state_nth: usize,
    pub(crate) contrast_set: bool,
}

#[derive(Debug)]
pub struct RuleStartState {
    pub(crate) base: BaseATNState,
    pub(crate) rule_stop_state_nth: usize,
    pub(crate) contrast_set: bool,
    pub(crate) precedence: bool,
}

#[derive(Debug)]
pub struct RuleStopState {
    pub(crate) base: BaseATNState,
}

#[derive(Debug)]
pub struct PlusBlockStartState {
    pub(crate) base: BlockStartState,
    pub(crate) plus_loopback_state_nth: usize,
    pub(crate) contrast_set: bool,
}

#[derive(Debug)]
pub struct StarBlockStartState {
    pub(crate) base: BlockStartState,
}

#[derive(Debug)]
pub struct TokenStartState {
    pub(crate) base: DecisionState,
}

#[derive(Debug)]
pub struct StarLoopbackState {
    pub(crate) base: BaseATNState,
}

#[derive(Debug)]
pub struct StarLoopEntryState {
    pub(crate) base: DecisionState,
    pub(crate) star_loopback_state_nth: usize,
    pub(crate) contrast_set: bool,
}

#[derive(Debug)]
pub struct PlusLoopbackState {
    pub(crate) base: DecisionState,
}

#[derive(Debug)]
pub struct LoopEndState {
    pub(crate) base: BaseATNState,
    pub(crate) loopback_state_nth: usize,
    pub(crate) contrast_set: bool,
}
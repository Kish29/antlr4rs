pub(crate) type LexerActionType = i32;

// transition types
const LEXER_ACTION_CHANNEL: LexerActionType = 0;
const LEXER_ACTION_CUSTOM: LexerActionType = 1;
const LEXER_ACTION_MODE: LexerActionType = 2;
const LEXER_ACTION_MORE: LexerActionType = 3;
const LEXER_ACTION_POP_MODE: LexerActionType = 4;
const LEXER_ACTION_PUSH_MODE: LexerActionType = 5;
const LEXER_ACTION_SKIP: LexerActionType = 6;
const LEXER_ACTION_TYPE: LexerActionType = 7;

// Although using one structure for each type looks very redundant,
// please don't optimize it yet in order to keep the code stylistically uniform
#[derive(Debug)]
pub enum LexerAction {
    Channel(ChannelLexerAction),
    Custom(CustomLexerAction),
    Mode(ModeLexerAction),
    More(MoreLexerAction),
    PopMode(PopModeLexerAction),
    PushMode(PushModeLexerAction),
    Skip(SkipLexerAction),
    Type(TypeLexerAction),
    IndexedCustom(IndexedCustomLexerAction),
}

impl LexerAction {
    // #[inline(always)]
    pub fn new(action_type: LexerActionType, data1: isize, data2: isize) -> Self {
        match action_type {
            LEXER_ACTION_CHANNEL => {
                LexerAction::Channel(ChannelLexerAction {
                    base: BaseLexerAction::new(LEXER_ACTION_CHANNEL),
                    channel: data1,
                })
            }
            LEXER_ACTION_CUSTOM => {
                LexerAction::Custom(CustomLexerAction {
                    base: BaseLexerAction::new(LEXER_ACTION_CUSTOM),
                    rule_idx: data1,
                    action_idx: data2,
                })
            }
            LEXER_ACTION_MODE => {
                LexerAction::Mode(ModeLexerAction {
                    base: BaseLexerAction::new(LEXER_ACTION_MODE),
                    mode: data1,
                })
            }
            LEXER_ACTION_MORE => {
                LexerAction::More(MoreLexerAction::new(LEXER_ACTION_MORE))
            }
            LEXER_ACTION_POP_MODE => {
                LexerAction::PopMode(PopModeLexerAction::new(LEXER_ACTION_POP_MODE))
            }
            LEXER_ACTION_PUSH_MODE => {
                LexerAction::PushMode(PushModeLexerAction {
                    base: BaseLexerAction::new(LEXER_ACTION_PUSH_MODE),
                    mode: data1,
                })
            }
            LEXER_ACTION_SKIP => {
                LexerAction::Skip(SkipLexerAction::new(LEXER_ACTION_SKIP))
            }
            LEXER_ACTION_TYPE => {
                LexerAction::Type(TypeLexerAction {
                    base: BaseLexerAction::new(LEXER_ACTION_TYPE),
                    the_type: data1,
                })
            }
            _ => panic!("lexer action type {} is invalid", action_type)
        }
    }
}

#[derive(Debug)]
pub struct BaseLexerAction {
    action_type: LexerActionType,
}

impl BaseLexerAction {
    // #[inline(always)]
    fn new(action_type: LexerActionType) -> Self { Self { action_type } }
}

#[derive(Debug)]
pub struct ChannelLexerAction {
    pub(crate) base: BaseLexerAction,
    pub(crate) channel: isize,
}

#[derive(Debug)]
pub struct CustomLexerAction {
    pub(crate) base: BaseLexerAction,
    pub(crate) rule_idx: isize,
    pub(crate) action_idx: isize,
}

#[derive(Debug)]
pub struct ModeLexerAction {
    pub(crate) base: BaseLexerAction,
    pub(crate) mode: isize,
}

pub type MoreLexerAction = BaseLexerAction;

pub type PopModeLexerAction = BaseLexerAction;

#[derive(Debug)]
pub struct PushModeLexerAction {
    pub(crate) base: BaseLexerAction,
    pub(crate) mode: isize,
}

pub type SkipLexerAction = BaseLexerAction;

#[derive(Debug)]
pub struct TypeLexerAction {
    pub(crate) base: BaseLexerAction,
    pub(crate) the_type: isize,
}

#[derive(Debug)]
pub struct IndexedCustomLexerAction {
    pub(crate) base: BaseLexerAction,
    pub(crate) offset: isize,
    pub(crate) lexer_action_nth: usize,
    pub(crate) contrast_set: bool,
}

impl IndexedCustomLexerAction {
    // #[inline(always)]
    pub fn new(offset: isize, la_type: LexerActionType, la_nth: usize) -> Self {
        Self {
            base: BaseLexerAction::new(la_type),
            offset,
            lexer_action_nth: la_nth,
            contrast_set: true,
        }
    }
}
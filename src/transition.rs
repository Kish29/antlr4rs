use std::rc::Rc;
use crate::interval_set::IntervalSet;
use crate::token::TOKEN_EOF;

pub(crate) type TransitionType = i32;

// transition types
const TRANSITION_EPSILON: TransitionType = 1;
const TRANSITION_RANGE: TransitionType = 2;
const TRANSITION_RULE: TransitionType = 3;
const TRANSITION_PREDICATE: TransitionType = 4;
const TRANSITION_ATOM: TransitionType = 5;
const TRANSITION_ACTION: TransitionType = 6;
const TRANSITION_SET: TransitionType = 7;
const TRANSITION_NOT_SET: TransitionType = 8;
const TRANSITION_WILDCARD: TransitionType = 9;
const TRANSITION_PRECEDENCE: TransitionType = 10;

#[derive(Debug)]
pub struct BaseTransition {
    // ATN state position in ATN's states
    pub(crate) target_nth: usize,
    trans_type: TransitionType,
}

impl BaseTransition {
    // #[inline(always)]
    pub fn new(atn_state_nth: usize, trans_type: TransitionType) -> Self {
        Self { target_nth: atn_state_nth, trans_type }
    }
}

// also use enumerations to decrease dynamic cost when types are fixed
#[derive(Debug)]
pub enum Transition {
    Epsilon(EpsilonTransition),
    Range(RangeTransition),
    Rule(RuleTransition),
    Predicate(PredicateTransition),
    Atom(AtomTransition),
    Action(ActionTransition),
    Set(SetTransition),
    NotSet(NotSetTransition),
    Wildcard(WildcardTransition),
    Precedence(PrecedenceTransition),
}

impl Transition {
    // #[inline(always)]
    pub fn new(ttype: TransitionType, trg: usize, arg1: isize, arg2: isize, arg3: isize, sets: &Vec<Rc<IntervalSet>>) -> Self {
        match ttype {
            TRANSITION_EPSILON => Self::new_epsilon(trg, -1),
            TRANSITION_RANGE => {
                if arg3 != 0 {
                    Self::new_range(trg, TOKEN_EOF, arg2)
                } else {
                    Self::new_range(trg, arg1, arg2)
                }
            }
            TRANSITION_RULE => Self::new_rule(arg1 as usize, arg2, arg3, trg),
            TRANSITION_PREDICATE => Self::new_predicate(trg, arg1, arg2, arg3 != 0),
            TRANSITION_ATOM => {
                if arg3 != 0 {
                    Self::new_atom(trg, TOKEN_EOF)
                } else {
                    Self::new_atom(trg, arg1)
                }
            }
            TRANSITION_ACTION => Self::new_action(trg, arg1, arg2, arg3 != 0),
            TRANSITION_SET => Self::new_set(trg, Rc::clone(&sets[arg1 as usize])),
            TRANSITION_NOT_SET => Self::new_not_set(trg, Rc::clone(&sets[arg1 as usize])),
            TRANSITION_WILDCARD => Self::new_wildcard(trg),
            TRANSITION_PRECEDENCE => Self::new_precedence(trg, arg1),
            _ => panic!("transition type {} is invalid", ttype)
        }
    }

    // #[inline(always)]
    pub fn target_nth(&self) -> usize {
        match self {
            Transition::Epsilon(e) => e.base.target_nth,
            Transition::Range(r) => r.base.target_nth,
            Transition::Rule(r) => r.base.target_nth,
            Transition::Predicate(p) => p.base.target_nth,
            Transition::Atom(a) => a.base.target_nth,
            Transition::Action(a) => a.base.target_nth,
            Transition::Set(s) => s.base.target_nth,
            Transition::NotSet(ns) => ns.base.target_nth,
            Transition::Wildcard(w) => w.base.target_nth,
            Transition::Precedence(p) => p.base.target_nth,
        }
    }

    // #[inline(always)]
    pub fn is_epsilon(&self) -> bool {
        match self {
            Transition::Epsilon(_) |
            Transition::Rule(_) |
            Transition::Predicate(_) |
            Transition::Action(_) |
            Transition::Precedence(_) => true,
            _ => false,
        }
    }

    // #[inline(always)]
    pub fn new_epsilon(trg: usize, opr: isize) -> Self {
        let base = BaseTransition::new(trg, TRANSITION_EPSILON);
        if opr != -1 {
            println!("create new_epislon!!!: {}", opr);
        }
        Transition::Epsilon(EpsilonTransition { base, outermost_precedence_return: opr })
    }

    // #[inline(always)]
    fn new_range(trg: usize, start: isize, stop: isize) -> Self {
        let base = BaseTransition::new(trg, TRANSITION_RANGE);
        Transition::Range(RangeTransition { base, start, stop })
    }

    // #[inline(always)]
    fn new_rule(trg: usize, rule_idx: isize, precedence: isize, follow_state_nth: usize) -> Self {
        let base = BaseTransition::new(trg, TRANSITION_RULE);
        Transition::Rule(RuleTransition { base, follow_state_nth, rule_idx, precedence })
    }

    // #[inline(always)]
    fn new_predicate(trg: usize, rule_idx: isize, pre_idx: isize, is_ctx_dependent: bool) -> Self {
        let base = BaseTransition::new(trg, TRANSITION_PREDICATE);
        Transition::Predicate(PredicateTransition { base, rule_idx, pre_idx, is_ctx_dependent })
    }

    // #[inline(always)]
    fn new_atom(trg: usize, label: isize) -> Self {
        let base = BaseTransition::new(trg, TRANSITION_ATOM);
        Transition::Atom(AtomTransition { base, label })
    }

    // #[inline(always)]
    fn new_action(trg: usize, rule_idx: isize, action_idx: isize, is_ctx_dependent: bool) -> Self {
        let base = BaseTransition::new(trg, TRANSITION_ACTION);
        Transition::Action(ActionTransition { base, rule_idx, action_idx, is_ctx_dependent })
    }

    // #[inline(always)]
    fn new_set(trg: usize, interval_set: Rc<IntervalSet>) -> Self {
        let base = BaseTransition::new(trg, TRANSITION_SET);
        Transition::Set(SetTransition { base, interval_set })
    }

    // #[inline(always)]
    fn new_not_set(trg: usize, interval_set: Rc<IntervalSet>) -> Self {
        let base = BaseTransition::new(trg, TRANSITION_NOT_SET);
        Transition::NotSet(NotSetTransition { base, interval_set })
    }

    // #[inline(always)]
    fn new_wildcard(trg: usize) -> Self {
        let base = BaseTransition::new(trg, TRANSITION_WILDCARD);
        Transition::Wildcard(WildcardTransition { base })
    }

    // #[inline(always)]
    fn new_precedence(trg: usize, precedence: isize) -> Self {
        let base = BaseTransition::new(trg, TRANSITION_PRECEDENCE);
        Transition::Precedence(PrecedenceTransition { base, precedence })
    }
}

#[derive(Debug)]
pub struct EpsilonTransition {
    pub(crate) base: BaseTransition,
    pub(crate) outermost_precedence_return: isize,
}

#[derive(Debug)]
pub struct RangeTransition {
    pub(crate) base: BaseTransition,
    pub(crate) start: isize,
    pub(crate) stop: isize,
}

#[derive(Debug)]
pub struct RuleTransition {
    pub(crate) base: BaseTransition,
    pub(crate) follow_state_nth: usize,
    rule_idx: isize,
    pub(crate) precedence: isize,
}

#[derive(Debug)]
pub struct PredicateTransition {
    pub(crate) base: BaseTransition,
    is_ctx_dependent: bool,
    rule_idx: isize,
    pre_idx: isize,
}

#[derive(Debug)]
pub struct AtomTransition {
    pub(crate) base: BaseTransition,
    label: isize,
}

#[derive(Debug)]
pub struct ActionTransition {
    pub(crate) base: BaseTransition,
    is_ctx_dependent: bool,
    rule_idx: isize,
    action_idx: isize,
}

#[derive(Debug)]
pub struct SetTransition {
    pub(crate) base: BaseTransition,
    pub(crate) interval_set: Rc<IntervalSet>,
}

#[derive(Debug)]
pub struct NotSetTransition {
    pub(crate) base: BaseTransition,
    pub(crate) interval_set: Rc<IntervalSet>,
}

#[derive(Debug)]
pub struct WildcardTransition {
    pub(crate) base: BaseTransition,
}

#[derive(Debug)]
pub struct PrecedenceTransition {
    pub(crate) base: BaseTransition,
    pub(crate) precedence: isize,
}

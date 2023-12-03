/// [ATNDeserializer] deserialize i32 array into ATN struct.
/// i32 array place data in this order
/// | serialized-version, atn-states-num, {atn-state-type, rule-index, {contrast-nth}\*}\*, non-greedy-states-num, {non-greedy-nth}\*, precedence-states-num, {precedence-nth}\* |
use std::slice::Iter;
use crate::atn::ATN;
use crate::atn_deserialize_option::ATNDeserializeOption;
use crate::atn_state::{ATNState, StateType};
use crate::atn_type::ATNType;
use crate::interval_set::IntervalSet;

const SERIALIZED_VERSION: isize = 1;

#[derive(Debug)]
pub struct ATNDeserializer {
    des_opt: ATNDeserializeOption,
}

impl ATNDeserializer {
    #[inline]
    pub fn new(options: Option<ATNDeserializeOption>) -> Self {
        Self { des_opt: options.unwrap_or(ATNDeserializeOption::default()) }
    }

    #[inline]
    pub fn deserialize(&self, data: &[i32]) -> ATN {
        let mut data = data.iter();
        self.check_version(&mut data);
        let mut atn = self.read_atn(&mut data);
        // parse atn states
        self.read_states(&mut data, &mut atn);
        // parse atn rules
        self.read_rules(&mut data, &mut atn);
        // parse modes
        self.read_modes(&mut data, &mut atn);
        // parse interval sets and read edges
        let sets = self.read_sets(&mut data);
        self.read_edges(&mut data, &mut atn, sets);
        atn
    }

    #[inline(always)]
    fn check_version(&self, data: &mut Iter<i32>) {
        let ver = data.next().unwrap();
        if (*ver as isize) != SERIALIZED_VERSION {
            panic!("Could not deserialize ATN with version {} (expected {}.)", ver, SERIALIZED_VERSION)
        }
    }

    #[inline(always)]
    fn read_atn(&self, data: &mut Iter<i32>) -> ATN {
        ATN::new(
            match *data.next().unwrap() {
                0 => ATNType::Lexer,
                1 => ATNType::Parser,
                _ => panic!("Invalid ATN type.")
            },
            *data.next().unwrap() as isize,
        )
    }

    /// parse all states into ATN, use enum type to implement in Rust to decrease dynamic or vtable cost.
    /// just store the nth for contrast ATN state type, how clever I am! ^_^
    #[inline(always)]
    fn read_states(&self, data: &mut Iter<i32>, atn: &mut ATN) {
        let states_num = *data.next().unwrap() as usize;
        atn.states = Vec::with_capacity(states_num);

        for nth in 0..states_num {
            // get atn state type.
            let state_type = *data.next().unwrap() as StateType;
            // check type, panic if type is invalid.
            ATNState::check_type(state_type);
            // get rule index
            let rule_idx = *data.next().unwrap() as usize;
            // create a new atn state
            let mut atn_state = ATNState::new(state_type, rule_idx, nth);
            // push anchors if atn state type is loop end or block start
            if let ATNState::LoopEnd(le) = &mut atn_state {
                le.loopback_state_nth = *data.next().unwrap() as usize;
                le.contrast_set = true;
            } else if atn_state.instance_of_block_start() {
                let bs = atn_state.get_block_start_mut().unwrap();
                bs.block_end_state_nth = *data.next().unwrap() as usize;
                bs.contrast_set = true;
            }
            atn.states.push(atn_state);
        }

        // check whether all block start and loopback has been set to it's peer correctly.
        self.check_contrast_states(&atn.states);

        let non_greedy_states_num = *data.next().unwrap() as usize;
        for _ in 0..non_greedy_states_num {
            let nth = *data.next().unwrap() as usize;
            atn.states[nth].to_decision_state_mut().unwrap().non_greedy = true;
        }

        let precedence_states_num = *data.next().unwrap() as usize;
        for _ in 0..precedence_states_num {
            let nth = *data.next().unwrap() as usize;
            atn.states[nth].to_rule_start_state_mut().unwrap().precedence = true;
        }
    }

    #[inline(always)]
    fn check_contrast_states(&self, states: &Vec<ATNState>) {
        for atn_state in states {
            match atn_state {
                ATNState::BlockStart(bs) => {
                    if !bs.contrast_set {
                        panic!("BlockStart's BlockEndState nth not set, it should not happen, it's a bug.")
                    }
                }
                ATNState::LoopEnd(le) => {
                    if !le.contrast_set {
                        panic!("LoopEnd's loopback nth not set, it should not happen, it's a bug.")
                    }
                }
                _ => ()
            }
        }
    }

    #[inline(always)]
    fn read_rules(&self, data: &mut Iter<i32>, atn: &mut ATN) {
        let rules_num = *data.next().unwrap() as usize;
        if atn.grammar_type == ATNType::Lexer {
            // reallocate the vec for elements, assign a new vec with specify capacity is faster than resize(my guess :) )
            atn.rule2token_type = Vec::with_capacity(rules_num);
        }

        for _ in 0..rules_num {
            let rs_nth = *data.next().unwrap() as usize;
            atn.rule2start_state_nths.push(rs_nth);

            if atn.grammar_type == ATNType::Lexer {
                let token_type = *data.next().unwrap() as usize;
                atn.rule2token_type.push(token_type);
            }
        }

        atn.rule2stop_state_nths.resize(rules_num, 0);

        for nth in 0..atn.states.len() {
            let mut is_stop = false;
            let mut stop_rule_idx = -1;
            if let ATNState::RuleStop(r) = &atn.states[nth] {
                is_stop = true;
                stop_rule_idx = r.base.rule_idx as i32;
            }
            if !is_stop {
                continue;
            }
            atn.rule2stop_state_nths[stop_rule_idx as usize] = nth;
            // hahaha, what's this?
            let rule_start = (&mut atn.states[atn.rule2start_state_nths[stop_rule_idx as usize]]).to_rule_start_state_mut().unwrap();
            rule_start.rule_stop_state_nth = nth;
            rule_start.contrast_set = true;
        }

        self.check_contrast_rules(&mut atn.states);
    }

    #[inline(always)]
    fn check_contrast_rules(&self, states: &Vec<ATNState>) {
        for atn_state in states {
            match atn_state {
                ATNState::RuleStart(r) => {
                    if !r.contrast_set {
                        panic!("RuleStart's RuleStopState nth not set, it should not happen, it's a bug.")
                    }
                }
                _ => ()
            }
        }
    }

    #[inline(always)]
    fn read_modes(&self, data: &mut Iter<i32>, atn: &mut ATN) {
        let modes_num = *data.next().unwrap() as usize;
        atn.mode2start_state_nths = Vec::with_capacity(modes_num);

        for _ in 0..modes_num {
            let token_start_state_nth = *data.next().unwrap() as usize;
            atn.mode2start_state_nths.push(token_start_state_nth);
        }
    }

    #[inline(always)]
    fn read_sets(&self, data: &mut Iter<i32>) -> Vec<IntervalSet> {
        let sets_num = *data.next().unwrap() as usize;
        let mut sets: Vec<IntervalSet> = Vec::with_capacity(sets_num);

        for _ in 0..sets_num {
            let mut set = IntervalSet::new();

            let n = *data.next().unwrap() as usize;
            let contains_eof = *data.next().unwrap();

            if contains_eof != 0 {
                set.add_one(-1);
            }

            for _ in 0..n {
                set.add_range(
                    *data.next().unwrap() as isize,
                    *data.next().unwrap() as isize,
                );
            }
            sets.push(set)
        }

        sets
    }

    #[inline(always)]
    fn read_edges(&self, data: &mut Iter<i32>, atn: &mut ATN, sets: Vec<IntervalSet>) {}
}
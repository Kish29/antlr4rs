/// [ATNDeserializer] deserialize i32 array into ATN struct.
/// i32 array place data in this order
/// | serialized-version, atn-states-num, {atn-state-type, rule-index, {contrast-nth}\*}\*, non-greedy-states-num, {non-greedy-nth}\*, precedence-states-num, {precedence-nth}\* |
use std::slice::Iter;
use crate::atn::ATN;
use crate::atn_deserialize_option::ATNDeserializeOption;
use crate::atn_state::{ATNState, StateType};
use crate::atn_type::ATNType;

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
            match &mut atn_state {
                ATNState::BlockStart(bs) => {
                    bs.block_end_state_nth = *data.next().unwrap() as usize;
                    bs.contrast_set = true;
                }
                ATNState::LoopEnd(le) => {
                    le.loopback_state_nth = *data.next().unwrap() as usize;
                    le.contrast_set = true;
                }
                _ => (),
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
}
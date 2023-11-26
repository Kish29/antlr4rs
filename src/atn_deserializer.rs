use std::intrinsics::pref_align_of;
/// [ATNDeserializer] deserialize i32 array into ATN struct.
/// i32 array place data in this order
/// | serialized-version, atn-states-num, {state-type, rule-index}* |
use std::slice::Iter;
use crate::atn::ATN;
use crate::atn_deserialize_option::ATNDeserializeOption;
use crate::atn_state::{ATN_STATE_BASIC, ATN_STATE_BLOCK_START, ATN_STATE_LOOP_END, ATNState, StateType};
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

    #[inline]
    fn read_states(&self, data: &mut Iter<i32>, atn: &mut ATN) {
        let states_num = *data.next().unwrap() as usize;
        atn.states = Vec::with_capacity(states_num);
        for i in 0..states_num {
            let state_type = *data.next().unwrap() as StateType;
            if state_type < ATN_STATE_BASIC || state_type > ATN_STATE_LOOP_END {
                panic!("deserialize an invalid ATN state type, it should not happen, it's a bug.")
            }
            let rule_idx = *data.next().unwrap() as usize;
            let atn_state = ATNState::new(state_type, rule_idx, i);
            if (state_type as StateType) == ATN_STATE_LOOP_END {
                todo!()
            } else if (state_type as StateType) == ATN_STATE_BLOCK_START {
                todo!()
            }
            atn.states.push(atn_state);
        }
    }
}
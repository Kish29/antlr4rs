use std::rc::Rc;
/// [ATNDeserializer] deserialize i32 array into ATN struct.
/// i32 array place data in this order
/// | serialized-version, atn-states-num, {atn-state-type, rule-index, {contrast-nth}\*}\*, non-greedy-states-num, {non-greedy-nth}\*, precedence-states-num, {precedence-nth}\* |
use std::slice::Iter;
use crate::atn::ATN;
use crate::atn_deserialize_option::ATNDeserializeOption;
use crate::atn_state::{ATNState, ATNStateType};
use crate::atn_type::ATNType;
use crate::interval_set::IntervalSet;
use crate::lexer_action::LexerAction;
use crate::transition::{Transition, TransitionType};

const SERIALIZED_VERSION: isize = 4;

#[derive(Debug)]
pub struct ATNDeserializer {
    des_opt: ATNDeserializeOption,
}

impl ATNDeserializer {
    // #[inline]
    pub fn new(options: Option<ATNDeserializeOption>) -> Self {
        Self { des_opt: options.unwrap_or(ATNDeserializeOption::default()) }
    }

    // #[inline]
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
        // parse decisions
        self.read_decisions(&mut data, &mut atn);
        // parse lexer actions
        self.read_lexer_actions(&mut data, &mut atn);
        self.mark_precedence_decisions(&mut atn);
        self.generate_rule_bypass_transition(&mut atn);
        self.verify_atn(&atn);
        atn
    }

    // #[inline(always)]
    fn check_version(&self, data: &mut Iter<i32>) {
        let ver = data.next().unwrap();
        if (*ver as isize) != SERIALIZED_VERSION {
            panic!("Could not deserialize ATN with version {} (expected {}.)", ver, SERIALIZED_VERSION)
        }
    }

    // #[inline(always)]
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
    // #[inline(always)]
    fn read_states(&self, data: &mut Iter<i32>, atn: &mut ATN) {
        let states_num = *data.next().unwrap() as usize;
        atn.states = Vec::with_capacity(states_num);

        for nth in 0..states_num {
            // get atn state type.
            let state_type = *data.next().unwrap() as ATNStateType;
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
                let bs = atn_state.to_block_start_mut().unwrap();
                bs.block_end_state_nth = *data.next().unwrap() as usize;
                bs.contrast_set = true;
            }
            atn.states.push(atn_state);
        }

        // check whether all block start and loopback has been set to it's peer correctly.
        let non_greedy_states_num = *data.next().unwrap() as usize;
        for _ in 0..non_greedy_states_num {
            let nth = *data.next().unwrap() as usize;
            atn.states[nth].to_decision_state_mut().unwrap().non_greedy = true;
        }

        let precedence_states_num = *data.next().unwrap() as usize;
        for _ in 0..precedence_states_num {
            let nth = *data.next().unwrap() as usize;
            atn.states[nth].to_rule_start_state_mut().unwrap().left_recursive = true;
        }
    }

    // #[inline(always)]
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
            let mut trg_rule_idx = 0;
            if let ATNState::RuleStop(r) = &atn.states[nth] {
                is_stop = true;
                trg_rule_idx = r.base.rule_idx;
            }
            if !is_stop {
                continue;
            }
            atn.rule2stop_state_nths[trg_rule_idx] = nth;
            // hahaha, what's this?
            let rule_start = (&mut atn.states[atn.rule2start_state_nths[trg_rule_idx]]).to_rule_start_state_mut().unwrap();
            rule_start.rule_stop_state_nth = nth;
            rule_start.contrast_set = true;
        }
    }

    // #[inline(always)]
    fn read_modes(&self, data: &mut Iter<i32>, atn: &mut ATN) {
        let modes_num = *data.next().unwrap() as usize;
        atn.mode2start_state_nths = Vec::with_capacity(modes_num);

        for _ in 0..modes_num {
            let token_start_state_nth = *data.next().unwrap() as usize;
            atn.mode2start_state_nths.push(token_start_state_nth);
        }
    }

    // #[inline(always)]
    fn read_sets(&self, data: &mut Iter<i32>) -> Vec<Rc<IntervalSet>> {
        let sets_num = *data.next().unwrap() as usize;
        let mut sets: Vec<Rc<IntervalSet>> = Vec::with_capacity(sets_num);

        for _ in 0..sets_num {
            let mut set = IntervalSet::new();

            let n = *data.next().unwrap() as usize;
            let contains_eof = *data.next().unwrap();

            if contains_eof != 0 {
                set.add_one(-1);
            }

            // in antlr4 for Golang runtime, it written: addRange(l, h+1)
            // cause Golang slice cut off not support nums[l:=h]
            // that's: Golang not support right index element included
            // but rust can do it, so just add_range(l, h). :-)
            for _ in 0..n {
                set.add_range(
                    *data.next().unwrap() as isize,
                    *data.next().unwrap() as isize,
                );
            }
            sets.push(Rc::new(set))
        }

        sets
    }

    // #[inline(always)]
    fn read_edges(&self, data: &mut Iter<i32>, atn: &mut ATN, sets: Vec<Rc<IntervalSet>>) {
        // Thank you Rust borrow checker :), you drove me mad when I write these trashes.

        let edges_num = *data.next().unwrap() as usize;
        for _ in 0..edges_num {
            let src = *data.next().unwrap() as usize;
            let trg = *data.next().unwrap() as usize;
            let ttype = *data.next().unwrap() as TransitionType;
            let arg1 = *data.next().unwrap() as isize;
            let arg2 = *data.next().unwrap() as isize;
            let arg3 = *data.next().unwrap() as isize;

            // create a new transition
            let transition = Transition::new(ttype, trg, arg1, arg2, arg3, &sets);
            atn.states[src].add_transition(transition, -1);
        }

        let mut trans_insert = Vec::with_capacity(atn.states.len() << 1);

        // Edges for rule stop states can be derived, so they are not serialized
        for state in &atn.states {
            for trans in state.transitions() {
                if let Transition::Rule(rt) = trans {
                    let mut outermost_precedence_return: isize = -1;

                    let trg_rule_idx = atn.states[rt.base.target_nth].rule_index();
                    let rule_start_atn_nth = atn.rule2start_state_nths[trg_rule_idx];

                    let rule_start_state = atn.states[rule_start_atn_nth].to_rule_start_state().unwrap();
                    if rule_start_state.left_recursive && rt.precedence == 0 {
                        outermost_precedence_return = trg_rule_idx as isize;
                    }
                    let ep = Transition::new_epsilon(rt.follow_state_nth, outermost_precedence_return);

                    trans_insert.push((atn.rule2stop_state_nths[trg_rule_idx], ep));
                }
            }
        }
        trans_insert.drain(..).for_each(|(idx, trans)| atn.states[idx].add_transition(trans, -1));

        let mut plus_block_start_modifies = Vec::with_capacity(atn.states.len() << 1);
        let mut star_loop_entry_modifies = Vec::with_capacity(atn.states.len() << 1);
        for nth in 0..atn.states.len() {
            if atn.states[nth].instance_of_block_start() {
                // We need to know the end state to set its start state
                let mut block_end_state_nth = 0;
                {
                    let bs = atn.states[nth].to_block_start().unwrap();
                    if !bs.contrast_set {
                        panic!("BlockStart contrast: BlockEnd pos not set.");
                    }
                    block_end_state_nth = bs.block_end_state_nth
                }
                // Block end states can only be associated to a single block start state
                let be = atn.states[block_end_state_nth].to_block_end_mut().unwrap();
                if be.contrast_set {
                    panic!("BlockEnd contrast: BlockStart pos already set.");
                }
                be.block_start_state_nth = nth;
                be.contrast_set = true;
            }

            let state = &atn.states[nth];

            if let ATNState::PlusLoopback(pl) = state {
                for trans in &pl.base.base.transitions {
                    let trg_nth = trans.target_nth();
                    if let ATNState::PlusBlockStart(_) = &atn.states[trg_nth] {
                        plus_block_start_modifies.push((trg_nth, nth));
                    }
                }
            } else if let ATNState::StarLoopback(sl) = state {
                for trans in &sl.base.transitions {
                    let trg_nth = trans.target_nth();
                    if let ATNState::StarLoopEntry(_) = &atn.states[trg_nth] {
                        star_loop_entry_modifies.push((trg_nth, nth));
                    }
                }
            }
        }

        if plus_block_start_modifies.len() != 0 {
            for (state_ntn, contrast_nth) in plus_block_start_modifies {
                let mut pbs = atn.states[state_ntn].to_plus_block_start_mut().unwrap();
                pbs.plus_loopback_state_nth = contrast_nth;
                pbs.contrast_set = true;
            }
        }

        if star_loop_entry_modifies.len() != 0 {
            for (state_ntn, contrast_nth) in star_loop_entry_modifies {
                let mut sle = atn.states[state_ntn].to_star_loop_entry_mut().unwrap();
                sle.star_loopback_state_nth = contrast_nth;
                sle.contrast_set = true;
            }
        }
    }

    // #[inline(always)]
    fn read_decisions(&self, data: &mut Iter<i32>, atn: &mut ATN) {
        let decisions_num = *data.next().unwrap() as usize;
        atn.decision2state_nth = Vec::with_capacity(decisions_num);

        for i in 0..decisions_num {
            let pos = *data.next().unwrap() as usize;
            atn.decision2state_nth.push(pos);
            atn.states[pos].to_decision_state_mut().unwrap().decision = i as isize;
        }
    }

    // #[inline(always)]
    fn read_lexer_actions(&self, data: &mut Iter<i32>, atn: &mut ATN) {
        if atn.grammar_type != ATNType::Lexer {
            return;
        }
        let actions_num = *data.next().unwrap() as usize;
        atn.lexer_actions = Vec::with_capacity(actions_num);

        for _ in 0..actions_num {
            let action_type = *data.next().unwrap();
            let data1 = *data.next().unwrap() as isize;
            let data2 = *data.next().unwrap() as isize;
            atn.lexer_actions.push(LexerAction::new(action_type, data1, data2));
        }
    }

    /// [ATNDeserializer::mark_precedence_decisions] analyzes the [ATNState::StarLoopEntry] states in the
    /// specified [ATN] to set the [ATNState::StarLoopEntry].precedence_decision field to the correct value.
    // #[inline(always)]
    fn mark_precedence_decisions(&self, atn: &mut ATN) {
        let mut star_loop_entry_modifies = Vec::with_capacity(atn.states.len());
        for nth in 0..atn.states.len() {
            let state = &atn.states[nth];
            // shit codes here, :(
            if let ATNState::StarLoopEntry(sle) = state {
                let rule_start_nth = atn.rule2start_state_nths[sle.base.base.rule_idx];
                let trg_state = &atn.states[rule_start_nth].to_rule_start_state().unwrap();
                if trg_state.left_recursive {
                    let maybe_loop_end_state_nth = state.transitions().last().unwrap().target_nth();
                    if let ATNState::LoopEnd(le) = &atn.states[maybe_loop_end_state_nth] {
                        let rs_nth = le.base.transitions.first().unwrap().target_nth();
                        if let ATNState::RuleStop(_) = &atn.states[rs_nth] {
                            star_loop_entry_modifies.push(nth)
                        }
                    }
                }
            }
        }

        for nth in star_loop_entry_modifies {
            let sle = atn.states[nth].to_star_loop_entry_mut().unwrap();
            sle.precedence_decision = true;
        }
    }

    // #[inline(always)]
    fn generate_rule_bypass_transition(&self, atn: &mut ATN) {
        if !self.des_opt.gen_rule_bypass_transitions {
            return;
        }
        todo!()
    }

    // #[inline(always)]
    fn verify_atn(&self, atn: &ATN) {
        if !self.des_opt.verify_atn {
            return;
        }
        for state in &atn.states {
            let base;
            match state {
                ATNState::Basic(b) => {
                    base = b;
                }
                ATNState::RuleStart(rs) => {
                    base = &rs.base;
                    if !rs.contrast_set {
                        panic!("RuleStart's RuleStopState nth not set, it should not happen, it's a bug.");
                    }
                }
                ATNState::BlockStart(bs) => {
                    base = &bs.base.base;
                    if !bs.contrast_set {
                        panic!("BlockStart's BlockEndState nth not set, it should not happen, it's a bug.")
                    }
                }
                ATNState::PlusBlockStart(pbs) => {
                    base = &pbs.base.base.base;
                    if !pbs.contrast_set {
                        panic!("PlusBlockStart's PlusLoopback nth not set, it should not happen, it's a bug.")
                    }
                }
                ATNState::StarBlockStart(sbs) => {
                    base = &sbs.base.base.base;
                }
                ATNState::TokenStart(ts) => {
                    base = &ts.base.base;
                }
                ATNState::RuleStop(rs) => {
                    base = &rs.base;
                }
                ATNState::BlockEnd(be) => {
                    base = &be.base;
                    if !be.contrast_set {
                        panic!("BlockEnd's BlockStartState nth not set, it should not happen, it's a bug.")
                    }
                }
                ATNState::StarLoopback(slb) => {
                    base = &slb.base;
                    if base.transitions.len() != 1 {
                        panic!("StarLoopback transition must have only one transition.")
                    }
                    match &atn.states[base.transitions[0].target_nth()] {
                        ATNState::StarLoopEntry(_) => (),
                        _ => panic!("StarLoopback transition must be StarLoopEntry.")
                    }
                }
                ATNState::StarLoopEntry(sle) => {
                    base = &sle.base.base;
                    if !sle.contrast_set {
                        panic!("StarLoopEntry's StarLoopback nth not set, it should not happen, it's a bug.")
                    }
                    if base.transitions.len() != 2 {
                        panic!("StarLoopEntry must have two transitions.")
                    }
                    match (&atn.states[base.transitions[0].target_nth()], &atn.states[base.transitions[1].target_nth()]) {
                        (ATNState::StarBlockStart(_), ATNState::LoopEnd(_)) => {
                            if sle.base.non_greedy {
                                panic!("StarLoopEntry (StarBlockStart, LoopEnd) should be Greedy.")
                            }
                        }
                        (ATNState::LoopEnd(_), ATNState::StarBlockStart(_)) => {
                            if !sle.base.non_greedy {
                                panic!("StarLoopEntry (LoopEnd, StarBlockStart) should be NonGreedy.")
                            }
                        }
                        _ => {
                            panic!("IllegalState has been insert into StarLoopEntry transitions.")
                        }
                    }
                }
                ATNState::PlusLoopback(plb) => {
                    base = &plb.base.base;
                }
                ATNState::LoopEnd(le) => {
                    base = &le.base;
                    if !le.contrast_set {
                        panic!("LoopEnd's loopback nth not set, it should not happen, it's a bug.")
                    }
                }
            }
            if !base.epsilon_only_trans && base.transitions.len() > 1 {
                panic!("atn state should only has one (epsilon) transition.")
            }
            if let Some(d) = state.to_decision_state() {
                if base.transitions.len() > 1 && d.decision < 0 {
                    panic!("DecisionState decision must set.")
                }
            }
        }
    }
}
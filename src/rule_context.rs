use crate::tree::RuleNode;

pub trait RuleContext: RuleNode {
    fn invoking_state(&self) -> isize;

    fn set_invoking_state(&mut self, s: isize);

    fn rule_index(&self) -> isize;

    fn is_empty(&self) -> bool;

    fn alt_number(&self) -> isize;

    fn set_alt_number(&mut self, alt_num: isize);

}
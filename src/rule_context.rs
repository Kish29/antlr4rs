use crate::tree::RuleNode;

pub trait RuleContext: RuleNode {
    fn invoking_state(&self) -> isize;

    fn set_invoking_state(&mut self, s: isize);

}
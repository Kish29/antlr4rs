use std::borrow::Cow;
use std::ops::Deref;
use std::rc::{Rc, Weak};
use crate::atn::ATN_INVALID_ALT_NUMBER;
use crate::tree::{ParseTree, ParseTreeVisitor, RuleNode, SyntaxTree, Tree};
use crate::value::Val;

pub trait RuleContext: RuleNode {
    fn invoking_state(&self) -> isize;

    fn set_invoking_state(&mut self, s: isize);

    fn rule_index(&self) -> isize;

    fn is_empty(&self) -> bool;

    fn alt_number(&self) -> isize;

    fn set_alt_number(&mut self, alt_num: isize);
}

pub struct BaseRuleContext {
    pub(crate) parent_ctx: Option<Weak<dyn RuleContext>>,
    pub(crate) invoking_state: isize,
    // pub(crate) rule_index: isize,
}

impl BaseRuleContext {
    // #[inline(always)]
    pub fn new(parent: Option<Rc<dyn RuleContext>>, invoking_state: isize) -> Self {
        Self { parent_ctx: parent.as_ref().map(Rc::downgrade), invoking_state }
    }
}

impl ParseTree for BaseRuleContext {
    // #[inline(always)]
    fn accept(&self, visitor: &dyn ParseTreeVisitor) -> Val {
        visitor.visit_children(self)
    }

    // #[inline]
    fn text(&self) -> Cow<'_, str> { Cow::Borrowed("") }
}

impl SyntaxTree for BaseRuleContext {
    // #[inline]
    fn source_start(&self) -> isize { 0 }

    // #[inline]
    fn source_end(&self) -> isize { 0 }
}

impl Tree for BaseRuleContext {
    // #[inline]
    fn parent(&self) -> Option<Rc<dyn Tree>> {
        match &self.parent_ctx {
            None => None,
            Some(p) => {
                p.upgrade().map(|ctx| ctx as Rc<dyn Tree>)
            }
        }
    }

    // #[inline]
    fn child(&self, _i: usize) -> Option<Rc<dyn Tree>> { None }

    // #[inline]
    fn child_count(&self) -> usize { 0 }
}

impl RuleContext for BaseRuleContext {
    // #[inline]
    fn invoking_state(&self) -> isize { self.invoking_state }

    // #[inline]
    fn set_invoking_state(&mut self, s: isize) { self.invoking_state = s }

    // #[inline]
    fn rule_index(&self) -> isize { 0 }

    // #[inline]
    fn is_empty(&self) -> bool { self.invoking_state == -1 }

    // #[inline]
    fn alt_number(&self) -> isize { ATN_INVALID_ALT_NUMBER }

    // #[inline]
    fn set_alt_number(&mut self, _alt_num: isize) {}
}
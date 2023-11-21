use std::borrow::Cow;
use std::rc::Rc;
use crate::error_listener::ErrorListener;
use crate::errors::ANTLRError;
use crate::rule_context::{BaseRuleContext, RuleContext};
use crate::token::Token;
use crate::tree::{ParseTree, ParseTreeVisitor, RuleNode, SyntaxTree, Tree};
use crate::value::Val;

pub trait ParserRuleContext: RuleContext {
    fn set_exception(&mut self, e: ANTLRError);

    fn enter_rule(&self, l: Rc<dyn ErrorListener>);
    fn exit_rule(&self, l: Rc<dyn ErrorListener>);

    fn start(&self) -> &dyn Token;
    fn set_start(&mut self, t: Rc<dyn Token>);

    fn stop(&self) -> &dyn Token;
    fn set_stop(&mut self, t: Rc<dyn Token>);

    fn add_child(&mut self, child: Rc<dyn RuleContext>) -> Rc<dyn RuleContext>;
    fn remove_last_child(&mut self);
}

pub struct BaseParserRuleContext {
    base: BaseRuleContext,

    pub(crate) start: Option<Rc<dyn Token>>,
    pub(crate) stop: Option<Rc<dyn Token>>,
    pub(crate) exception: Option<ANTLRError>,
    pub(crate) children: Vec<Rc<dyn ParserRuleContext>>,
}

impl BaseParserRuleContext {
    pub fn new(parent: Option<Rc<dyn ParserRuleContext>>, invoking_state: isize) -> Self {
        Self {
            base: BaseRuleContext::new(
                parent.map(|p| p as Rc<dyn RuleContext>),
                invoking_state,
            ),
            start: None,
            stop: None,
            exception: None,
            children: vec![],
        }
    }
}

impl RuleNode for BaseParserRuleContext {
    #[inline]
    fn rule_context(&self) -> &dyn RuleContext { self }
}

impl ParseTree for BaseParserRuleContext {
    #[inline(always)]
    fn accept(&self, visitor: &dyn ParseTreeVisitor) -> Val {
        visitor.visit_children(self)
    }

    #[inline]
    fn text(&self) -> Cow<'_, str> {
        if self.children.len() == 0 {
            return Cow::Borrowed("");
        }
        let mut s = String::new();
        for x in &self.children {
            s.push_str(x.text().as_ref());
        }
        Cow::Owned(s)
    }
}

impl SyntaxTree for BaseParserRuleContext {
    #[inline]
    fn source_start(&self) -> isize {
        match &self.start {
            None => -1,
            Some(stt) => stt.token_index()
        }
    }

    #[inline]
    fn source_end(&self) -> isize {
        match &self.stop {
            None => -2,
            Some(stp) => stp.token_index()
        }
    }
}

impl Tree for BaseParserRuleContext {
    fn parent(&self) -> Option<&dyn Tree> {
        self.base.parent()
    }

    #[inline]
    fn child(&self, i: usize) -> Option<&dyn Tree> {
        let cc = self.child_count();
        if cc == 0 || i >= cc {
            return None;
        }
        Some(self.children[i].as_ref() as &dyn Tree)
    }

    #[inline]
    fn child_count(&self) -> usize {
        self.children.len()
    }
}

impl RuleContext for BaseParserRuleContext {
    fn invoking_state(&self) -> isize {
        self.base.invoking_state()
    }

    fn set_invoking_state(&mut self, s: isize) {
        self.base.set_invoking_state(s)
    }

    fn rule_index(&self) -> isize {
        self.base.rule_index()
    }

    fn is_empty(&self) -> bool {
        self.base.is_empty()
    }

    fn alt_number(&self) -> isize {
        self.base.alt_number()
    }

    fn set_alt_number(&mut self, alt_num: isize) {
        self.base.set_alt_number(alt_num)
    }
}

impl ParserRuleContext for BaseParserRuleContext {
    fn set_exception(&mut self, e: ANTLRError) {
        todo!()
    }

    fn enter_rule(&self, l: Rc<dyn ErrorListener>) {
        todo!()
    }

    fn exit_rule(&self, l: Rc<dyn ErrorListener>) {
        todo!()
    }

    fn start(&self) -> &dyn Token {
        todo!()
    }

    fn set_start(&mut self, t: Rc<dyn Token>) {
        self.start = Some(t)
    }

    fn stop(&self) -> &dyn Token {
        todo!()
    }

    fn set_stop(&mut self, t: Rc<dyn Token>) {
        self.stop = Some(t)
    }

    fn add_child(&mut self, child: Rc<dyn RuleContext>) -> Rc<dyn RuleContext> {
        todo!()
    }

    fn remove_last_child(&mut self) {
        todo!()
    }
}
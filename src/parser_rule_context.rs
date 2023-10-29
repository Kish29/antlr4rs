use std::borrow::Cow;
use std::cell::RefCell;
use std::rc::Rc;
use crate::error_listener::ErrorListener;
use crate::errors::ANTLRError;
use crate::rule_context::RuleContext;
use crate::token::Token;
use crate::tree::{ParseTree, ParseTreeVisitor, RuleNode, SyntaxTree, Tree};
use crate::value::Val;

pub trait ParserRuleContext: RuleContext {
    fn set_exception(&mut self, e: ANTLRError);

    fn enter_rule(&self, l: Rc<dyn ErrorListener>);
    fn exit_rule(&self, l: Rc<dyn ErrorListener>);

    fn start(&self) -> Rc<dyn Token>;
    fn set_start(&self, t: Rc<dyn Token>);

    fn stop(&self) -> Rc<dyn Token>;
    fn set_stop(&self, t: Rc<dyn Token>);

    fn add_child(&mut self, child: Rc<dyn RuleContext>) -> Rc<dyn RuleContext>;
    fn remove_last_child(&mut self);
}

pub struct BaseParserRuleContext {
    pub(crate) parent_ctx: Rc<dyn RuleContext>,
    pub(crate) invoking_state: isize,

    pub rule_index: isize,

    pub(crate) start: Rc<dyn Token>,
    pub(crate) stop: Rc<dyn Token>,
    pub(crate) exception: Option<ANTLRError>,
    pub(crate) children: Vec<Rc<RefCell<dyn Tree>>>,
}

impl RuleNode for BaseParserRuleContext {
    fn rule_context(&self) -> &dyn RuleContext {
        todo!()
    }
}

impl ParseTree for BaseParserRuleContext {
    fn accept(&self, visitor: &dyn ParseTreeVisitor) -> Val {
        todo!()
    }

    fn text(&self) -> Cow<'_, str> {
        todo!()
    }
}

impl SyntaxTree for BaseParserRuleContext {
    fn source_start(&self) -> isize {
        todo!()
    }

    fn source_end(&self) -> isize {
        todo!()
    }
}

impl Tree for BaseParserRuleContext {
    fn payload(&self) -> Option<&dyn Tree> {
        todo!()
    }

    fn child(&self, i: isize) -> Option<&dyn Tree> {
        todo!()
    }

    fn child_count(&self) -> isize {
        todo!()
    }
}

impl RuleContext for BaseParserRuleContext {
    fn invoking_state(&self) -> isize {
        todo!()
    }

    fn set_invoking_state(&mut self, s: isize) {
        todo!()
    }

    fn rule_index(&self) -> isize {
        todo!()
    }

    fn is_empty(&self) -> bool {
        todo!()
    }

    fn alt_number(&self) -> isize {
        todo!()
    }

    fn set_alt_number(&mut self, alt_num: isize) {
        todo!()
    }
}
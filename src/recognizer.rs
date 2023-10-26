use std::cell::RefCell;
use std::rc::Rc;
use crate::antlr_error_listener::ANTLRErrorListener;
use crate::atn::ATN;
use crate::recognition_exception::RecognitionException;
use crate::rule_context::RuleContext;

pub trait Recognizer {
    fn literal_names(&self) -> &[&str];

    fn rule_names(&self) -> &[&str];

    fn sempred<T: RuleContext>(_local_ctx: &T, _rule_idx: isize, _action_idx: isize) -> bool { true }

    fn precpred<T: RuleContext>(_local_ctx: &T, _precedence: isize) -> bool { true }

    fn atn(&self) -> &ATN;

    fn state(&self) -> isize;

    fn set_state(&mut self, state: isize);

    fn action<T: RuleContext>(&self, _local_ctx: &T, _rule_idx: isize, _action_idx: isize) {}

    fn add_error_listener(&mut self, l: Rc<RefCell<dyn ANTLRErrorListener>>);

    fn remove_error_listeners(&mut self);

    fn error_listener_dispatch<T: ANTLRErrorListener>(&self) -> &mut T;

    fn has_error(&self) -> bool;

    fn error(&self) -> Option<Rc<dyn RecognitionException>>;

    fn set_error(&mut self, e: Box<dyn RecognitionException>);
}

pub struct BaseRecognizer {
    listeners: Vec<Rc<RefCell<dyn ANTLRErrorListener>>>,
}
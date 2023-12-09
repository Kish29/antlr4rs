use std::cell::RefCell;
use std::rc::Rc;
use crate::error_listener::ErrorListener;
use crate::errors::ANTLRError;
use crate::rule_context::RuleContext;

pub trait Recognizer {
    fn literal_names(&self) -> &[&str];

    fn symbolic_names(&self) -> &[&str];

    fn rule_names(&self) -> &[&str];

    fn sempred(&self, _local_ctx: Rc<dyn RuleContext>, _rule_idx: isize, _action_idx: isize) -> bool;

    fn precpred(&self, _local_ctx: Rc<dyn RuleContext>, _precedence: isize) -> bool;

    // fn atn(&self) -> &ATN;

    fn state(&self) -> isize;

    fn set_state(&mut self, state: isize);

    fn action(&self, _local_ctx: Rc<dyn RuleContext>, _rule_idx: isize, _action_idx: isize);

    fn add_error_listener(&mut self, l: Rc<RefCell<dyn ErrorListener>>);

    fn remove_error_listeners(&mut self);

    fn error_listener_dispatch(&self) -> Rc<RefCell<dyn ErrorListener>>;

    fn has_error(&self) -> bool;

    fn error(&self) -> Option<&ANTLRError>;

    fn set_error(&mut self, e: ANTLRError);
}

pub struct BaseRecognizer {
    listeners: Vec<Rc<dyn ErrorListener>>,
    state: isize,

    rule_names: &'static [&'static str],
    literal_names: &'static [&'static str],
    symbolic_names: &'static [&'static str],
    grammar_file_name: &'static str,
    syn_err: Option<ANTLRError>,
}

impl BaseRecognizer {
    // #[inline(always)]
    pub fn new(
        rule_names: &'static [&'static str],
        literal_names: &'static [&'static str],
        symbolic_names: &'static [&'static str],
        grammar_file_name: &'static str,
    ) -> Self {
        Self {
            listeners: vec![],
            state: -1,
            rule_names,
            literal_names,
            symbolic_names,
            grammar_file_name,
            syn_err: None,
        }
    }
}

impl Recognizer for BaseRecognizer {
    fn literal_names(&self) -> &[&str] {
        self.literal_names
    }

    fn symbolic_names(&self) -> &[&str] {
        self.symbolic_names
    }

    fn rule_names(&self) -> &[&str] {
        self.rule_names
    }

    fn sempred(&self, _local_ctx: Rc<dyn RuleContext>, _rule_idx: isize, _action_idx: isize) -> bool {
        todo!()
    }

    fn precpred(&self, _local_ctx: Rc<dyn RuleContext>, _precedence: isize) -> bool {
        todo!()
    }

    /*fn atn(&self) -> &ATN {
        todo!()
    }*/

    fn state(&self) -> isize {
        todo!()
    }

    fn set_state(&mut self, state: isize) {
        todo!()
    }

    fn action(&self, _local_ctx: Rc<dyn RuleContext>, _rule_idx: isize, _action_idx: isize) {
        todo!()
    }

    fn add_error_listener(&mut self, l: Rc<RefCell<dyn ErrorListener>>) {
        todo!()
    }

    fn remove_error_listeners(&mut self) {
        todo!()
    }

    fn error_listener_dispatch(&self) -> Rc<RefCell<dyn ErrorListener>> {
        todo!()
    }

    fn has_error(&self) -> bool {
        todo!()
    }

    fn error(&self) -> Option<&ANTLRError> {
        todo!()
    }

    fn set_error(&mut self, e: ANTLRError) {
        todo!()
    }
}
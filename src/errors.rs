use std::error::Error;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum ANTLRError {
    LexerNoAltErr {
        start_index: isize,
    },
    NoViableErr(NoViableError),
    InputMismatchErr(InputMismatchError),
    PredicateErr(PredicateError),
    OtherErr(Rc<dyn Error>),
}

#[derive(Clone, Debug)]
pub struct NoViableError {}

#[derive(Clone, Debug)]
pub struct InputMismatchError {}

#[derive(Clone, Debug)]
pub struct PredicateError {}
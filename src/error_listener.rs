use crate::errors::ANTLRError;
use crate::recognizer::Recognizer;

pub trait ErrorListener {
    fn syntax_error(
        &mut self,
        _recognizer: &dyn Recognizer,
        _line: isize,
        _column: isize,
        _msg: &str,
        _err: Option<&ANTLRError>,
    ) {}
}

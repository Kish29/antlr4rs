use crate::recognition_exception::RecognitionException;
use crate::recognizer::Recognizer;

pub trait ANTLRErrorListener {
    fn syntax_error<T, R: Recognizer, E: RecognitionException>(
        &mut self,
        _recognizer: &R,
        _offending_symbol: Option<&T>,
        _line: isize,
        _column: isize,
        _msg: &str,
        _err: Option<&E>,
    ) {}
}

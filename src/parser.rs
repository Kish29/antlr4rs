use crate::recognizer::Recognizer;

pub trait Parser: Recognizer {
    fn precedence(&self) -> isize;
}

pub struct BaseParser<R, PAS, TS>
where R: Recognizer,
    PAS:
{

    pub(crate) recognizer: R,
    pub(crate)

}
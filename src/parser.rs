use crate::parser_atn_simulator::ParserATNSimulator;
use crate::recognizer::Recognizer;

pub trait Parser: Recognizer {
    fn precedence(&self) -> isize;
}

pub struct BaseParser<R, PAS, TS>
    where R: Recognizer,
          PAS: ParserATNSimulator,
{
    pub(crate) recognizer: R,
    pub(crate) interpreter: PAS,
    pub(crate) token_stream: TS,
}
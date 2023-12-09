use crate::atn_simulator::{ATNSimulator, BaseATNSimulator};

pub trait ParserATNSimulator: ATNSimulator {
    // seems nothing to export here?
}

pub struct BaseParserATNSimulator {
    base: BaseATNSimulator,
}
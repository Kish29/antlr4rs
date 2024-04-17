use std::sync::Arc;
use crate::misc::murmur3::MurmurHash;
use crate::Nth;
use crate::prediction_context::PredictionContext;
use crate::semantic_context::SemanticContext;

pub enum ATNConfigType {
    LexerConfig,
    ParserConfig,
}

#[derive(Debug)]
pub struct ATNConfig {
    // The ATN state associated with this configuration
    state_nth: Nth,
    // What alt (or lexer rule) is predicted by this configuration
    alt: isize,
    // The stack of invoking states leading to the rule/states associated
    // with this config.  We track only those contexts pushed during
    // execution of the ATN simulator.
    context: Arc<PredictionContext>,
    pub semantic_context: SemanticContext,
    pub reaches_into_outer_context: isize,
}

impl PartialEq for ATNConfig {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}


impl MurmurHash for ATNConfig {
    // #[inline]
    fn murmur(&self) -> u32 {
        todo!()
    }
}
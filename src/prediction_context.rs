use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::Nth;

#[derive(Debug)]
pub enum PredictionContext {
    Empty,
    Singleton(SingletonPredictionContext),
    Array(ArrayPredictionContext),
}

#[derive(Debug)]
pub struct SingletonPredictionContext {
    cached_hash: u32,
    ret_state_nth: Nth,
    parent_ctx: Arc<PredictionContext>,
}

#[derive(Debug)]
pub struct ArrayPredictionContext {
    cached_hash: u32,
    ret_states_nths: Vec<Nth>,
    parents: Vec<Arc<PredictionContext>>,
}

#[derive(Debug)]
pub struct PredictionContextCache {
    pub(crate) cache: RwLock<HashMap<PredictionContext, PredictionContext>>,
}

impl PredictionContextCache {
    // #[inline(always)]
    pub fn new() -> Self {
        Self { cache: RwLock::new(HashMap::with_capacity(32)) }
    }
}
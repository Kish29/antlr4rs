use std::collections::HashMap;
use std::hash::Hash;
use std::sync::RwLock;

#[derive(Debug)]
pub enum PredictionContext {
    Empty(EmptyPredictionContext),
    Singleton(SingletonPredictionContext),
    Array(ArrayPredictionContext),
}

#[derive(Debug)]
pub struct EmptyPredictionContext {}

#[derive(Debug)]
pub struct SingletonPredictionContext {}

#[derive(Debug)]
pub struct ArrayPredictionContext {}

#[derive(Debug)]
pub struct PredictionContextCache {
    pub(crate) cache: RwLock<HashMap<PredictionContext, PredictionContext>>,
}

impl PredictionContextCache {
    #[inline(always)]
    pub fn new() -> Self {
        Self { cache: RwLock::new(HashMap::with_capacity(32)) }
    }
}
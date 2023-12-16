use crate::atn_config::ATNConfig;
use crate::misc::murmur3::HashCode;

/// implement set trait for atn configs
#[derive(Debug)]
pub struct ATNConfigSet {
    cached_hash: Option<u32>,
    pub read_only: bool,
    pub configs: Vec<ATNConfig>,
}

impl HashCode for ATNConfigSet {
    // #[inline]
    fn hash_code(&self) -> u32 {
        let mut h: u32 = 1;
        for ue in &self.configs {
            h = h.wrapping_mul(31).wrapping_add(ue.hash_code());
        }
        h
    }
}
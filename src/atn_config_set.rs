use crate::atn_config::ATNConfig;
use crate::misc::murmur3::MurmurHash;

/// implement set trait for atn configs
#[derive(Debug)]
pub struct ATNConfigSet {
    pub configs: Vec<ATNConfig>,

    // Indicates that this configuration set is part of a full context
    // LL prediction. It will be used to determine how to merge $. With SLL
    // it's a wildcard whereas it is not for LL context merge.
    pub full_ctx: bool,
}

impl ATNConfigSet {
    // #[inline(always)]
    pub fn new(full_ctx: bool) -> Self {
        Self { configs: vec![], full_ctx }
    }
}

impl MurmurHash for ATNConfigSet {
    // #[inline]
    fn murmur(&self) -> u32 {
        let mut h: u32 = 1;
        for ue in &self.configs {
            h = h.wrapping_mul(31).wrapping_add(ue.murmur());
        }
        h
    }
}
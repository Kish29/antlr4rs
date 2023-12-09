#[derive(Debug)]
pub struct ATNDeserializeOption {
    pub(crate) read_only: bool,
    pub(crate) verify_atn: bool,
    pub(crate) gen_rule_bypass_transitions: bool,
}

impl Default for ATNDeserializeOption {
    fn default() -> Self {
        Self { read_only: true, verify_atn: true, gen_rule_bypass_transitions: false }
    }
}
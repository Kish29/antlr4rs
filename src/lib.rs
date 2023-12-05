// It's 2023/11/19 when I add this feature to support trait upcasting in Antlr, that could:
// cast trait B to trait A which B extend from A, so I can cast parse rule context into tree or rule context.
// Fortunately, this feature will be stabilized soon at this point of time(Thanks for Rust teams),
// and I look forward to the day when this project finished and I can remove this annotation without concern.
// See issue: https://github.com/rust-lang/rust/issues/65991
// todo: remove this
#![feature(trait_upcasting)]
#![allow(incomplete_features)]

// todo: look closely at all function annotated by #[inline], consider using it properly.


pub mod atn;
pub mod atn_type;
pub mod recognizer;
pub mod lexer;
pub mod input_stream;
pub mod tree;
pub mod token;
pub mod int_stream;
pub mod token_stream;
pub mod token_source;
pub mod char_stream;
pub mod code_point;
pub mod atn_simulator;
pub mod prediction_context;
pub mod dfa;
pub mod dfa_state;
pub mod atn_config_set;
pub mod atn_config;
pub mod atn_state;
pub mod token_factory;
pub mod parser_rule_context;
pub mod rule_context;
pub mod value;
pub mod lexer_atn_simulator;
pub mod error_listener;
pub mod errors;
pub mod lexer_action;
pub mod parser;
pub mod any_ext;
pub mod common_token_stream;
mod parser_atn_simulator;
pub mod atn_deserializer;
pub mod atn_deserialize_option;
mod interval_set;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

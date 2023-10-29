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

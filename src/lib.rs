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
mod atn_simulator;
mod prediction_context;
mod dfa;
mod dfa_state;
mod atn_config_set;
mod atn_config;
mod atn_state;
pub mod token_factory;
pub mod parser_rule_context;
pub mod rule_context;
pub mod val;

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

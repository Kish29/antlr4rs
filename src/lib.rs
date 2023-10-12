pub mod atn;
pub mod tree;
pub mod token;
pub mod int_stream;
pub mod tokensrc;
pub mod chstream;
pub mod interval;
pub mod tokenstream;
pub mod chstreams;
pub mod codp_chstream;

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

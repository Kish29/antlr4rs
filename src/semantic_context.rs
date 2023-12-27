use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum SemanticContext {
    None,
    Predicate(PredicateSemanticContext),
    Precedence(PrecedenceSemanticContext),
    AND(ANDSemanticContext),
    OR(ORSemanticContext),
}

impl Display for SemanticContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SemanticContext::Predicate(p) => {
                f.write_fmt(format_args!("{{{}:{}}}", p.rule_index, p.pred_index))
            }
            SemanticContext::Precedence(p) => {
                f.write_fmt(format_args!("{{{}>=prec}}?", p))
            }
            SemanticContext::AND(a) => {
                let sstrs: Vec<String> = a.iter().map(|op| format!("{}", op)).collect();
                f.write_str(sstrs.join("&&").as_str())
            }
            SemanticContext::OR(o) => {
                let sstrs: Vec<String> = o.iter().map(|op| format!("{}", op)).collect();
                f.write_str(sstrs.join("||").as_str())
            }
            SemanticContext::None => {
                f.write_str("none")
            }
        }
    }
}

pub type PrecedenceSemanticContext = isize;

pub type ANDSemanticContext = Vec<SemanticContext>;
pub type ORSemanticContext = Vec<SemanticContext>;

#[derive(Debug)]
pub struct PredicateSemanticContext {
    rule_index: usize,
    pred_index: usize,
    ctx_dependent: bool,
}


use crate::rule_context::RuleContext;

pub trait ParserRuleContext: RuleContext {

    fn enter_rule();

}
use std::borrow::Cow;
use crate::rule_context::RuleContext;
use crate::token::Token;
use crate::value::Val;

pub trait Tree {
    /// The parent of this node. a
    /// If the return value is None, then this node is the root of the tree.
    fn parent(&self) -> Option<&dyn Tree> {
        None
    }

    fn payload(&self) -> Option<&dyn Tree>;

    /// If there are children, get the `i`-th value indexed from 0.
    fn child(&self, i: isize) -> Option<&dyn Tree>;

    fn child_count(&self) -> isize;
}

pub trait SyntaxTree: Tree {
    fn source_start(&self) -> isize;

    fn source_end(&self) -> isize;
}

pub trait ParseTree: SyntaxTree {
    fn accept(&self, visitor: &dyn ParseTreeVisitor) -> Val;

    fn text(&self) -> Cow<'_, str>;
}

pub trait RuleNode: ParseTree {
    fn rule_context(&self) -> &dyn RuleContext;
}

pub trait TerminalNode: ParseTree {
    fn symbol(&self) -> &dyn Token;
}

pub trait ErrorNode: TerminalNode {}

pub trait ParseTreeVisitor {
    fn visit(&self, tree: &dyn Tree) -> Val;

    fn visit_children(&self, node: &dyn RuleNode) -> Val;

    fn visit_terminal(&self, node: &dyn TerminalNode) -> Val;

    fn visit_err_node(&self, node: &dyn ErrorNode) -> Val;
}
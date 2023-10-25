use std::borrow::Cow;
use crate::rule_context::RuleContext;
use crate::token::Token;

pub trait Tree {
    /// The parent of this node. a
    /// If the return value is None, then this node is the root of the tree.
    fn parent<T: Tree>(&self) -> Option<&T> {
        None
    }

    fn payload<T: Tree>(&self) -> Option<&T>;

    /// If there are children, get the `i`-th value indexed from 0.
    fn child<T: Tree>(&self, i: isize) -> Option<&T>;

    fn child_count(&self) -> isize;
}

pub trait SyntaxTree: Tree {
    fn source_start(&self) -> isize;

    fn source_end(&self) -> isize;
}

pub trait ParseTree: SyntaxTree {
    fn accept<Ptv: ParseTreeVisitor>(&self, visitor: &Ptv) -> Ptv::Val;

    fn text(&self) -> Cow<'_, str>;
}

pub trait RuleNode: ParseTree {
    fn rule_context<Ctx: RuleContext>(&self) -> &Ctx;
}

pub trait TerminalNode: ParseTree {
    fn symbol<TK: Token>(&self) -> &TK;
}

pub trait ErrorNode: TerminalNode {}

pub trait ParseTreeVisitor {
    type Val;

    fn visit<T: Tree>(&self, tree: &T) -> Self::Val;

    fn visit_children<R: RuleNode>(&self, node: &R) -> Self::Val;

    fn visit_terminal<TN: TerminalNode>(&self, node: &TN) -> Self::Val;

    fn visit_err_node<E: ErrorNode>(&self, node: &E) -> Self::Val;
}
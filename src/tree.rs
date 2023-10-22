use std::borrow::Cow;
use std::rc::Rc;
use crate::rule_context::RuleContext;
use crate::token::Token;
use crate::val::Val;

pub trait Tree<'a> {
    /// The parent of this node. a
    /// If the return value is None, then this node is the root of the tree.
    fn parent(&self) -> Option<&mut dyn Tree> {
        None
    }

    fn payload(&self) -> Option<&mut dyn Tree>;

    /// If there are children, get the `i`-th value indexed from 0.
    fn child(&self, i: isize) -> Option<&mut dyn Tree>;

    fn child_count(&self) -> isize;
}

pub trait SyntaxTree<'a>: Tree<'a> {
    fn source_start(&self) -> isize;

    fn source_end(&self) -> isize;
}

pub trait ParseTree<'a>: SyntaxTree<'a> {
    fn accept(&mut self, visitor: &mut dyn ParseTreeVisitor) -> Val;

    fn text(&'a mut self) -> Cow<'a, str>;
}

pub trait RuleNode<'a>: ParseTree<'a> {
    fn rule_context(&mut self) -> &mut dyn RuleContext;
}

pub trait TerminalNode<'a>: ParseTree<'a> {
    fn symbol(&mut self) -> &mut dyn Token;
}

pub trait ErrorNode<'a>: TerminalNode<'a> {}

pub trait ParseTreeVisitor<'a> {
    fn visit(&mut self, tree: &mut dyn Tree) -> Val;

    fn visit_children(&mut self, node: &mut dyn RuleNode) -> Val;

    fn visit_terminal(&mut self, node: &mut dyn TerminalNode) -> Val;

    fn visit_err_node(&mut self, node: &mut dyn ErrorNode) -> Val;
}
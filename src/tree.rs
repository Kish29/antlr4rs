use std::fmt::{Debug, Display};
use std::rc::Rc;
use crate::val::Val;

pub trait Tree<'a> {
    /// The parent of this node. a
    /// If the return value is None, then this node is the root of the tree.
    fn parent(&self) -> Option<Box<Self>> {
        None
    }

    fn payload(&self) -> Option<Box<Self>>;

    /// If there are children, get the `i`-th value indexed from 0.
    fn child(&self, i: isize) -> Option<Box<Self>>;

    fn child_count(&self) -> isize;
}

pub trait SyntaxTree<'a>: Tree<'a> {
    fn source_start(&self) -> isize;

    fn source_end(&self) -> isize;
}

pub trait ParseTree<'a>: SyntaxTree<'a> {
    fn accept(&mut self, visitor: &mut dyn ParseTreeVisitor) -> Box<dyn Val>;
}

pub trait RuleNode<'a>: ParseTree<'a> {}

pub trait TerminalNode<'a>: ParseTree<'a> {}

pub trait ErrorNode<'a>: TerminalNode<'a> {}

pub trait ParseTreeVisitor<'a> {
    fn visit(&mut self, tree: &mut dyn Tree) -> Box<dyn Val>;

    fn visit_children(&mut self, node: &mut dyn RuleNode) -> Box<dyn Val>;

    fn visit_terminal(&mut self, node: &mut dyn TerminalNode) -> Box<dyn Val>;

    fn visit_err_node(&mut self, node: &mut dyn ErrorNode) -> Box<dyn Val>;
}
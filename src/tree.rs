use std::any::TypeId;
use std::borrow::Cow;
use std::fmt::Debug;
use crate::any_ext::AnyExt;
use crate::check_base;
use crate::rule_context::RuleContext;
use crate::token::Token;
use crate::value::Val;
use crate::value::Val::Nil;

// here defines all type that will use in parser progress
// have no choice to use dynamic type instead of use template cause using template is too complex to design.
// this may cause some performance decrease, or somebody can help me to implement this? :)


/// The basic notion of a tree has a parent, a payload, and a list of children.
/// It is the most abstract interface for all the trees used by ANTLR.
pub trait Tree: AnyExt {
    // fn tid(&self) -> usize { 0 }

    /// The parent of this node. a
    /// If the return value is None, then this node is the root of the tree.
    fn parent(&self) -> Option<&dyn Tree>;

    /// If there are children, get the `i`-th value indexed from 0.
    fn child(&self, i: usize) -> Option<&dyn Tree>;

    fn child_count(&self) -> usize;
}

/// A tree that knows about an interval in a token stream
/// is some kind of syntax tree. Subinterfaces distinguish
/// between parse trees and other kinds of syntax trees we might want to create.
pub trait SyntaxTree: Tree {
    fn source_start(&self) -> isize;

    fn source_end(&self) -> isize;
}

/// An interface to access the tree of [RuleContext] objects created
/// during a parse that makes the data structure look like a simple parse tree.
/// This node represents both internal nodes, rule invocations,
/// and leaf nodes, token matches.
/// The payload is either a [Token] or a [RuleContext] object.
pub trait ParseTree: SyntaxTree {
    fn accept(&self, visitor: &dyn ParseTreeVisitor) -> Val;

    fn text(&self) -> Cow<'_, str>;
}

/// An interface that represent the rule context.
pub trait RuleNode: ParseTree {
    fn rule_context(&self) -> &dyn RuleContext;
}

pub trait TerminalNode: ParseTree {
    fn symbol(&self) -> &dyn Token;
}

pub trait ErrorNode: TerminalNode {}

pub trait ParseTreeVisitor: AnyExt {
    fn visit(&self, tree: &dyn ParseTree) -> Val;

    fn visit_children(&self, node: &dyn RuleNode) -> Val;

    fn visit_terminal(&self, node: &dyn TerminalNode) -> Val;

    fn visit_err_node(&self, node: &dyn ErrorNode) -> Val;
}

#[derive(Debug)]
pub struct BaseParseTreeVisitor;

impl Default for BaseParseTreeVisitor {
    fn default() -> Self { Self {} }
}

impl AnyExt for BaseParseTreeVisitor {
    fn does_impl(&self, tid: &TypeId) -> bool {
        check_base!(self, tid);
        false
    }
}

impl ParseTreeVisitor for BaseParseTreeVisitor {
    // #[inline(always)]
    fn visit(&self, tree: &dyn ParseTree) -> Val { tree.accept(self) }

    fn visit_children(&self, _node: &dyn RuleNode) -> Val { Nil }

    fn visit_terminal(&self, _node: &dyn TerminalNode) -> Val { Nil }

    fn visit_err_node(&self, _node: &dyn ErrorNode) -> Val { Nil }
}

use std::any::Any;
use std::borrow::Cow;
use std::fmt::Debug;
use std::rc::Rc;
use crate::parser_rule_context::ParserRuleContext;
use crate::rule_context::RuleContext;
use crate::token::Token;
use crate::value::Val;
use crate::value::Val::Nil;

// here defines all type that will use in parser progress
// have no choice to use dynamic type instead of using template,
// cause using template is too complex to design.
// this may cause some performance decrease :(
// i'd appreciate it if somebody can help me to implement this by using template :)


/// The basic notion of a tree has a parent, a payload, and a list of children.
/// It is the most abstract interface for all the trees used by ANTLR.
pub trait Tree: Any + 'static {
    /// The parent of this node. a
    /// If the return value is None, then this node is the root of the tree.
    fn parent(&self) -> Option<Rc<dyn Tree>>;

    /// If there are children, get the `i`-th value indexed from 0.
    fn child(&self, i: usize) -> Option<Rc<dyn Tree>>;

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
    fn symbol(&self) -> Rc<dyn Token>;
}

pub trait ErrorNode: TerminalNode {}

pub trait ParseTreeListener: Any + 'static {
    fn visit_terminal(&self, _node: &dyn TerminalNode) -> Val { Nil }

    fn visit_error_node(&self, _node: &dyn TerminalNode) -> Val { Nil }

    fn enter_every_rule(&self, _ctx: &dyn ParserRuleContext) -> Val { Nil }

    fn exit_every_rule(&self, _ctx: &dyn ParserRuleContext) -> Val { Nil }
}

pub trait ParseTreeVisitor: Any + 'static {
    fn visit(&self, tree: &dyn ParseTree) -> Val;

    fn visit_children(&self, _node: &dyn RuleNode) -> Val { Nil }

    fn visit_terminal(&self, _node: &dyn TerminalNode) -> Val { Nil }

    fn visit_error_node(&self, _node: &dyn ErrorNode) -> Val { Nil }
}

impl<T: Any + 'static> ParseTreeVisitor for T {
    fn visit(&self, tree: &dyn ParseTree) -> Val {
        tree.accept(self)
    }
}
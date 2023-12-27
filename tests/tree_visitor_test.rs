#![feature(trait_upcasting)]
#![allow(incomplete_features)]

use std::any::Any;
use std::borrow::Cow;
use std::fmt::Debug;
use std::rc::Rc;
use antlr4rs::parser_rule_context::BaseParserRuleContext;
use antlr4rs::rule_context::RuleContext;
use antlr4rs::token::Token;
use antlr4rs::tree::{ErrorNode, ParseTree, ParseTreeVisitor, RuleNode, SyntaxTree, TerminalNode, Tree};
use antlr4rs::value::Val;
use antlr4rs::value::Val::{Nil, StrSRef};

struct MyParseTree {
    base: BaseParserRuleContext,
    text: String,
}

/*impl MyParseTree {
    pub fn accept_custom(&self, _visitor: &dyn ParseTreeVisitor) -> Val {
        StrSRef("antlr4rs")
    }
}

impl ErrorNode for MyParseTree {}

impl TerminalNode for MyParseTree {
    fn symbol(&self) -> Rc<dyn Token> { todo!() }
}

impl RuleNode for MyParseTree {
    fn rule_context(&self) -> &dyn RuleContext {
        todo!()
    }
}*/

impl Tree for MyParseTree {
    fn parent(&self) -> Option<Rc<dyn Tree>> {
        self.base.parent()
    }

    fn child(&self, i: usize) -> Option<Rc<dyn Tree>> {
        self.base.child(i)
    }

    fn child_count(&self) -> usize { self.base.child_count() }
}


impl SyntaxTree for MyParseTree {
    fn source_start(&self) -> isize { 0 }

    fn source_end(&self) -> isize { 0 }
}

impl ParseTree for MyParseTree {
    fn accept(&self, visitor: &dyn ParseTreeVisitor) -> Val {
        if let Some(mptv) = (visitor as &dyn Any).downcast_ref::<MyParseTreeVisitorProxy>() {
            println!("MyParseTree recognized MyParseTreeVisitorProxy");
            return mptv.visit_my_node(self);
        }
        self.base.accept(visitor)
    }

    fn text(&self) -> Cow<'_, str> {
        Cow::Borrowed(self.text.as_str())
    }
}

trait MyParseTreeVisitor {
    fn visit_my_node(&self, _my_node: &MyParseTree) -> Val { Nil }
}

struct MyParseTreeVisitorProxy {
    inner: Box<dyn MyParseTreeVisitor>,
}

impl MyParseTreeVisitorProxy {
    pub fn new<T: MyParseTreeVisitor + 'static>(v: T) -> Self {
        Self { inner: Box::new(v) }
    }

    pub fn visit_my_node(&self, my_node: &MyParseTree) -> Val {
        self.inner.visit_my_node(my_node)
    }
}

#[derive(Debug)]
struct AnyAnyAny {
    name: String,
    age: usize,
}

impl AnyAnyAny {
    fn new(name: String, age: usize) -> Self {
        Self { name, age }
    }
}

struct IamVisitor {}

impl MyParseTreeVisitor for IamVisitor {
    fn visit_my_node(&self, my_node: &MyParseTree) -> Val {
        println!("{}", &my_node.text);
        StrSRef("I am visitor implement generated trait.")
    }
}

#[test]
fn test_tree_visitor() {
    let mpt: &dyn ParseTree = &MyParseTree { base: BaseParserRuleContext::new(None, -1), text: "my parse tree visitor. :)".to_string() };
    let mptva: &dyn ParseTreeVisitor = &MyParseTreeVisitorProxy::new(IamVisitor {});
    println!("{:?}", mptva.visit(mpt));
}
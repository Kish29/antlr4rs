#![feature(trait_upcasting)]
#![allow(incomplete_features)]

use std::any::Any;
use std::borrow::Cow;
use std::fmt::Debug;
use std::rc::Rc;
use antlr4rs::input_stream::StringStream;
use antlr4rs::parser_rule_context::BaseParserRuleContext;
use antlr4rs::rule_context::RuleContext;
use antlr4rs::token::Token;
use antlr4rs::token_factory::{CommonTokenFactory, TokenFactory};
use antlr4rs::tree::{BaseParseTreeVisitor, ErrorNode, ParseTree, ParseTreeVisitor, RuleNode, SyntaxTree, TerminalNode, Tree};
use antlr4rs::value::{StructType, Val};
use antlr4rs::value::Val::{Arr, Bool, Struct, Str, Int64, Uint64, AnyBox, StrSRef};

struct MyParseTree {
    base: BaseParserRuleContext,
    text: String,
}

impl MyParseTree {
    pub fn accept_custom(&self, visitor: &dyn ParseTreeVisitor) -> Val {
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
}

impl SyntaxTree for MyParseTree {
    fn source_start(&self) -> isize {
        0
    }

    fn source_end(&self) -> isize {
        0
    }
}

impl Tree for MyParseTree {
    fn parent(&self) -> Option<Rc<dyn Tree>> {
        self.base.parent()
    }

    fn child(&self, i: usize) -> Option<Rc<dyn Tree>> {
        self.base.child(i)
    }

    fn child_count(&self) -> usize { self.base.child_count() }
}

impl ParseTree for MyParseTree {
    fn accept(&self, visitor: &dyn ParseTreeVisitor) -> Val {
        if let Some(mptv) = (visitor as &dyn Any).downcast_ref::<MyParseTreeVisitor>() {
            println!("MyParseTree recognized MyParseTreeVisitor");
            return mptv.visit_custom(self);
        }
        self.base.accept(visitor)
    }

    fn text(&self) -> Cow<'_, str> {
        Cow::Borrowed(self.text.as_str())
    }
}

#[derive(Debug)]
struct MyParseTreeVisitor {
    base: BaseParseTreeVisitor,
}

impl MyParseTreeVisitor {
    pub fn visit_custom(&self, _tree: &dyn ParseTree) -> Val {
        Str("visit_custom".to_string())
    }
}

impl ParseTreeVisitor for MyParseTreeVisitor {
    fn visit(&self, tree: &dyn ParseTree) -> Val {
        if let Some(mp) = (tree as &dyn Any).downcast_ref::<MyParseTree>() {
            println!("visit tree type is MyParseTree, id: {:?}", tree.type_id());
            return mp.accept_custom(self);
        }
        tree.accept(self)
    }

    fn visit_children(&self, node: &dyn RuleNode) -> Val {
        Arr(vec![Str("1".to_string()), Bool(true), Int64(128)])
    }

    fn visit_terminal(&self, node: &dyn TerminalNode) -> Val {
        let mut obj = StructType::new();
        obj.insert("name".to_string(), Str("Jack".to_string()));
        obj.insert("age".to_string(), Uint64(24));
        obj.insert("any type".to_string(), AnyBox(Box::new(AnyAnyAny::new(String::from("Kish29"), 24))));
        Struct(obj)
    }

    fn visit_err_node(&self, node: &dyn ErrorNode) -> Val {
        Uint64(128)
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

#[test]
fn test_tree_visitor() {
    let mpt = MyParseTree { base: BaseParserRuleContext::new(None, -1), text: "my parse tree visitor. :)".to_string() };
    let mptv = MyParseTreeVisitor { base: Default::default() };
    println!("{:?}", mpt.accept(&mptv));
    println!("{:?}", mptv.visit(&mpt));
    println!("{:?}", mptv.visit_children(&mpt));
    println!("{:?}", mptv.visit_terminal(&mpt));
    println!("{:?}", mptv.visit_err_node(&mpt));
    println!("{:?}", mpt.text());

    let mut input = StringStream::from("input stream in test");

    let tf: CommonTokenFactory = CommonTokenFactory::new();
    let tk = tf.create(&mut input, 1, None, 1, 0, 4, 0, 0);
    println!("{:?}", tk.text());
}
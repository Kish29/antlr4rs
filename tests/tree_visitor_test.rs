use std::borrow::Cow;
use antlr4rs::input_stream::StringStream;
use antlr4rs::rule_context::RuleContext;
use antlr4rs::token::Token;
use antlr4rs::token_factory::{CommonTokenFactory, TokenFactory};
use antlr4rs::tree::{ErrorNode, ParseTree, ParseTreeVisitor, RuleNode, SyntaxTree, TerminalNode, Tree};
use antlr4rs::val::Number::{Int, UInt};
use antlr4rs::val::{Object, Val};
use antlr4rs::val::Val::{Arr, Bool, Num, Obj, Str};

struct MyParseTree {
    text: String,
}

impl<'a> ErrorNode<'a> for MyParseTree {}

impl<'a> TerminalNode<'a> for MyParseTree {
    fn symbol(&mut self) -> &mut dyn Token {
        todo!()
    }
}

impl<'a> RuleNode<'a> for MyParseTree {
    fn rule_context(&mut self) -> &mut dyn RuleContext {
        todo!()
    }
}

impl<'a> SyntaxTree<'a> for MyParseTree {
    fn source_start(&self) -> isize {
        0
    }

    fn source_end(&self) -> isize {
        0
    }
}

impl<'a> Tree<'a> for MyParseTree {
    fn payload(&self) -> Option<&mut dyn Tree> {
        None
    }

    fn child(&self, i: isize) -> Option<&mut dyn Tree> {
        None
    }

    fn child_count(&self) -> isize {
        0
    }
}

impl<'a> ParseTree<'a> for MyParseTree {
    fn accept(&mut self, visitor: &mut dyn ParseTreeVisitor) -> Val {
        visitor.visit(self as &mut dyn Tree)
    }

    fn text(&'a mut self) -> Cow<'a, str> {
        Cow::Borrowed(self.text.as_str())
    }
}

struct MyParseTreeVisitor {}

impl<'a> ParseTreeVisitor<'a> for MyParseTreeVisitor {
    fn visit(&mut self, tree: &mut dyn Tree) -> Val {
        Str("visit".to_string())
    }

    fn visit_children(&mut self, node: &mut dyn RuleNode) -> Val {
        Arr(vec![Str("1".to_string()), Bool(true), Num(Int(128))])
    }

    fn visit_terminal(&mut self, node: &mut dyn TerminalNode) -> Val {
        let mut obj = Object::new();
        obj.insert("name".to_string(), Str("Jack".to_string()));
        obj.insert("age".to_string(), Num(UInt(24)));
        Obj(obj)
    }

    fn visit_err_node(&mut self, node: &mut dyn ErrorNode) -> Val {
        Num(UInt(128))
    }
}

#[test]
fn test_tree_visitor() {
    let mut mpt = MyParseTree { text: "my parse tree visitor. :)".to_string() };
    let mut mptv = MyParseTreeVisitor {};
    println!("{:?}", mpt.accept(&mut mptv));
    println!("{:?}", mptv.visit(&mut mpt));
    println!("{:?}", mptv.visit_children(&mut mpt as &mut dyn RuleNode));
    println!("{:?}", mptv.visit_terminal(&mut mpt as &mut dyn TerminalNode));
    println!("{:?}", mptv.visit_err_node(&mut mpt as &mut dyn ErrorNode));
    println!("{:?}", mpt.text());

    let mut input = StringStream::new("input stream in test".to_string());

    let tf: CommonTokenFactory = Default::default();
    let tk = tf.create(&mut input, 1, None, 1, 0, 4, 0, 0);
    println!("{:?}", tk.text());
}
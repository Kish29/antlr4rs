use std::borrow::Cow;
use antlr4rs::input_stream::StringStream;
use antlr4rs::rule_context::RuleContext;
use antlr4rs::token::Token;
use antlr4rs::token_factory::{CommonTokenFactory, TokenFactory};
use antlr4rs::tree::{ErrorNode, ParseTree, ParseTreeVisitor, RuleNode, SyntaxTree, TerminalNode, Tree};
use antlr4rs::value::Number::{Int, UInt};
use antlr4rs::value::{Object, Val};
use antlr4rs::value::Val::{Arr, Bool, Num, Obj, Str};

struct MyParseTree {
    text: String,
}

impl ErrorNode for MyParseTree {}

impl TerminalNode for MyParseTree {
    fn symbol(&self) -> &dyn Token {
        todo!()
    }
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
    fn payload(&self) -> Option<&dyn Tree> {
        None
    }

    fn child(&self, i: isize) -> Option<&dyn Tree> {
        None
    }

    fn child_count(&self) -> isize {
        0
    }
}

impl ParseTree for MyParseTree {
    fn accept(&self, visitor: &dyn ParseTreeVisitor) -> Val {
        visitor.visit(self)
    }

    fn text(&self) -> Cow<'_, str> {
        Cow::Borrowed(self.text.as_str())
    }
}

struct MyParseTreeVisitor {}

impl ParseTreeVisitor for MyParseTreeVisitor {
    fn visit(&self, tree: &dyn Tree) -> Val {
        Str("visit".to_string())
    }

    fn visit_children(&self, node: &dyn RuleNode) -> Val {
        Arr(vec![Str("1".to_string()), Bool(true), Num(Int(128))])
    }

    fn visit_terminal(&self, node: &dyn TerminalNode) -> Val {
        let mut obj = Object::new();
        obj.insert("name".to_string(), Str("Jack".to_string()));
        obj.insert("age".to_string(), Num(UInt(24)));
        Obj(obj)
    }

    fn visit_err_node(&self, node: &dyn ErrorNode) -> Val {
        Num(UInt(128))
    }
}

#[test]
fn test_tree_visitor() {
    let mpt = MyParseTree { text: "my parse tree visitor. :)".to_string() };
    let mptv = MyParseTreeVisitor {};
    println!("{:?}", mpt.accept(&mptv));
    println!("{:?}", mptv.visit(&mpt));
    println!("{:?}", mptv.visit_children(&mpt));
    println!("{:?}", mptv.visit_terminal(&mpt));
    println!("{:?}", mptv.visit_err_node(&mpt));
    println!("{:?}", mpt.text());

    let mut input = StringStream::new("input stream in test".to_string());

    let tf: CommonTokenFactory = Default::default();
    let tk = tf.create(&mut input, 1, None, 1, 0, 4, 0, 0);
    println!("{:?}", tk.text());
}
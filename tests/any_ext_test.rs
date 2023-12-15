#![feature(trait_upcasting)]
#![allow(incomplete_features)]

use std::any::TypeId;
use std::mem;
use antlr4rs::any_ext::AnyExt;
use antlr4rs::{check_base, downcast_trait_ref};
use antlr4rs::rule_context::RuleContext;
use antlr4rs::tree::{ErrorNode, ParseTree, RuleNode, SyntaxTree, TerminalNode, Tree};

#[test]
fn test_trait_id() {
    let tid = TypeId::of::<dyn Tree>();
    println!("{:?}", tid);
    let tid = TypeId::of::<dyn SyntaxTree>();
    println!("{:?}", tid);
    let tid = TypeId::of::<dyn ParseTree>();
    println!("{:?}", tid);
    let tid = TypeId::of::<dyn RuleNode>();
    println!("{:?}", tid);
    let tid = TypeId::of::<dyn TerminalNode>();
    println!("{:?}", tid);
    let tid = TypeId::of::<dyn ErrorNode>();
    println!("{:?}", tid);
    let tid = TypeId::of::<dyn RuleContext>();
    println!("{:?}", tid);
}

struct MyTree {}

impl AnyExt for MyTree {
    fn does_impl(&self, tid: &TypeId) -> bool {
        check_base!(tid);
        false
    }
}

impl Tree for MyTree {
    fn parent(&self) -> Option<&dyn Tree> {
        None
    }

    fn child(&self, i: usize) -> Option<&dyn Tree> {
        None
    }

    fn child_count(&self) -> usize { 0 }
}

impl SyntaxTree for MyTree {
    fn source_start(&self) -> isize { 0 }

    fn source_end(&self) -> isize { 0 }
}

#[test]
fn test_it_is() {
    let t: &dyn Tree = &MyTree {};
    println!("{:?}", TypeId::of::<dyn SyntaxTree>());
    if let Some(x) = downcast_trait_ref!(t, SyntaxTree) {}
    // assert_eq!(it_is::<dyn Tree, MyTree>(t), true);
    let t: &dyn SyntaxTree = &MyTree {};
    // assert_eq!(it_is::<dyn SyntaxTree, MyTree>(t), true);
    let t: Box<dyn SyntaxTree> = Box::new(MyTree {});
    // assert_eq!(it_is::<dyn SyntaxTree, MyTree>(t.as_ref()), true);
}

use std::any::TypeId;
use antlr4rs::tree::Tree;

#[test]
fn print_trait_id() {
    let tid = TypeId::of::<dyn Tree>();
    println!("{:?}", tid);
}
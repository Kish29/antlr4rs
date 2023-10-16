use std::any::Any;

pub trait Tree {
    /// The parent of this node. a
    /// If the return value is None, then this node is the root of the tree.
    fn parent(&self) -> Option<&dyn Tree> {
        None
    }

    fn payload(&self) -> Option<&dyn Any> {
        unimplemented!()
    }

    /// If there are children, get the `i`-th value indexed from 0.
    fn child(&self, i: isize) -> Option<&dyn Tree>;

    fn child_count(&self) -> isize;
}
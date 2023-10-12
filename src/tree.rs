use std::any::Any;
use std::rc::Rc;

pub trait Tree {
    /// The parent of this node. a
    /// If the return value is None, then this node is the root of the tree.
    fn parent() -> Option<Rc<Self>> {
        None
    }

    fn payload() -> Box<dyn Any> {
        unimplemented!()
    }


    /// If there are children, get the i-th value indexed from 0.
    fn child(i: isize) -> Option<Rc<dyn Tree>>;

    fn child_count() -> isize;

}
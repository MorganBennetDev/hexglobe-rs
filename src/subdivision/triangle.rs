use std::rc::Rc;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct Triangle<T: Clone> {
    pub u: Rc<T>,
    pub v: Rc<T>,
    pub w: Rc<T>
}

impl<T: Clone> Triangle<T> {
    pub const fn new(u: Rc<T>, v: Rc<T>, w: Rc<T>) -> Self {
        Self { u, v, w }
    }
}
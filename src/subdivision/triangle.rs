#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct Triangle<T: Clone> {
    pub u: T,
    pub v: T,
    pub w: T
}

impl<T: Clone> Triangle<T> {
    pub const fn new(u: T, v: T, w: T) -> Self {
        Self { u, v, w }
    }
}
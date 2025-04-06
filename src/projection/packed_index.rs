use std::fmt::{Debug, Formatter};
use petgraph::adj::IndexType;

#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PackedIndex(usize);

impl PackedIndex {
    pub const fn new(face: usize, subdivision: usize) -> Self {
        Self((subdivision << 5) | face)
    }
    
    pub const fn face(&self) -> usize {
        self.0 & 0b11111
    }
    
    pub const fn subdivision(&self) -> usize {
        self.0 >> 5
    }
}

impl Debug for PackedIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}:{:?}", self.face(), self.subdivision())
    }
}

unsafe impl IndexType for PackedIndex {
    fn new(x: usize) -> Self {
        Self(x)
    }
    
    fn index(&self) -> usize {
        self.0
    }
    
    fn max() -> Self {
        Self(usize::MAX)
    }
}

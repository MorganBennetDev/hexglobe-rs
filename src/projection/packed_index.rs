use petgraph::adj::IndexType;

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
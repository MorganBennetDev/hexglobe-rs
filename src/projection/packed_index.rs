use std::fmt::{Debug, Formatter};

/// Wrapper type for a `usize` in which the lower 5 bits represent the index of 1 of 20 icosahedral faces and the
/// remaining bits represent the index of a triangular subdivision within that face.
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PackedIndex(usize);

impl PackedIndex {
    /// Packs the specified indices into one value.
    pub const fn new(face: usize, subdivision: usize) -> Self {
        Self((subdivision << 5) | face)
    }
    
    /// Retrieves the stored face index.
    pub const fn face(&self) -> usize {
        self.0 & 0b11111
    }
    
    /// Retrieves the stored subdivision index.
    pub const fn subdivision(&self) -> usize {
        self.0 >> 5
    }
}

impl Debug for PackedIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}:{:?}", self.face(), self.subdivision())
    }
}
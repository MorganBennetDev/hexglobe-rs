use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ImplicitDenominator<T, const N: u32>(T);

impl<T, const N: u32> ImplicitDenominator<T, N> {
    const DENOMINATOR: u32 = N;
    
    pub fn new(value: T) -> Self {
        Self(value)
    }
}

impl<T, const N: u32> ImplicitDenominator<T, N> {
    pub const fn wrap(v: T) -> Self {
        Self(v)
    }
}

impl<T, const N: u32> Deref for ImplicitDenominator<T, N> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const N: u32> DerefMut for ImplicitDenominator<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, const N: u32> Debug for ImplicitDenominator<T, N> where
    T : Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
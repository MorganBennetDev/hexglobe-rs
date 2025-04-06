use std::fmt::{Debug, Formatter};
use std::ops::{Add, Div, Mul};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ImplicitDenominator<T, const N: u32>(pub T);

impl<T, const N: u32> ImplicitDenominator<T, N> {
    pub fn new(value: T) -> Self {
        Self(value)
    }
}

impl<T, const N: u32> ImplicitDenominator<T, N> {
    pub const fn wrap(v: T) -> Self {
        Self(v)
    }
}

impl<T, U, const N: u32> AsRef<U> for ImplicitDenominator<T, N> where
    T : AsRef<U> {
    fn as_ref(&self) -> &U {
        self.0.as_ref()
    }
}

impl<T, const N: u32> Debug for ImplicitDenominator<T, N> where
    T : Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}/{:?}", self.0, N)
    }
}

impl<T, U, const N: u32> Add<ImplicitDenominator<U, N>> for ImplicitDenominator<T, N> where
    T : Add<U> {
    type Output = ImplicitDenominator<<T as Add<U>>::Output, N>;
    
    fn add(self, rhs: ImplicitDenominator<U, N>) -> Self::Output {
        ImplicitDenominator(self.0 + rhs.0)
    }
}

impl<'a, T, U, const N: u32> Add<&'a ImplicitDenominator<U, N>> for ImplicitDenominator<T, N> where
    T : Add<U>,
    U : Clone {
    type Output = ImplicitDenominator<<T as Add<U>>::Output, N>;
    
    fn add(self, rhs: &'a ImplicitDenominator<U, N>) -> Self::Output {
        ImplicitDenominator(self.0 + rhs.0.clone())
    }
}

impl<'a, T, U, const N: u32> Add<ImplicitDenominator<U, N>> for &'a ImplicitDenominator<T, N> where
    T : Add<U> + Clone {
    type Output = ImplicitDenominator<<T as Add<U>>::Output, N>;
    
    fn add(self, rhs: ImplicitDenominator<U, N>) -> Self::Output {
        ImplicitDenominator(self.0.clone() + rhs.0)
    }
}

impl<'a, 'b, T, U, const N: u32> Add<&'a ImplicitDenominator<U, N>> for &'b ImplicitDenominator<T, N> where
    T : Add<U> + Clone,
    U : Clone {
    type Output = ImplicitDenominator<<T as Add<U>>::Output, N>;
    
    fn add(self, rhs: &'a ImplicitDenominator<U, N>) -> Self::Output {
        ImplicitDenominator(self.0.clone() + rhs.0.clone())
    }
}

impl<T, U, const N: u32> Mul<U> for ImplicitDenominator<T, N> where
    T : Mul<U> {
    type Output = ImplicitDenominator<<T as Mul<U>>::Output, N>;
    
    fn mul(self, rhs: U) -> Self::Output {
        ImplicitDenominator(self.0 * rhs)
    }
}

impl<'a, T, U, const N: u32> Mul<U> for &'a ImplicitDenominator<T, N> where
    T : Mul<U> + Clone {
    type Output = ImplicitDenominator<<T as Mul<U>>::Output, N>;
    
    fn mul(self, rhs: U) -> Self::Output {
        ImplicitDenominator(self.0.clone() * rhs)
    }
}

impl<T, U, const N: u32> Div<U> for ImplicitDenominator<T, N> where
    T : Div<U> {
    type Output = ImplicitDenominator<<T as Div<U>>::Output, N>;
    
    fn div(self, rhs: U) -> Self::Output {
        ImplicitDenominator(self.0 / rhs)
    }
}
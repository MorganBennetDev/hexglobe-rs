use std::fmt::{Debug, Formatter};
use std::ops::{Add, Div, Mul};

pub const fn max(a: usize, b: usize) -> usize {
    if a < b {
        b
    } else {
        a
    }
}

/*
Bits increase in significance left to right
 */
#[derive(Copy, Clone, Eq, Hash)]
pub struct FixedPoint<const N: usize>(u32);

impl<const DENOMINATOR: usize> FixedPoint<DENOMINATOR> {
    const BITS: usize = 32;
    const POINT: usize = DENOMINATOR.ilog2() as usize;
    
    pub fn new(n: u32) -> Self {
        let log2 = DENOMINATOR.ilog2();
        assert_eq!((DENOMINATOR >> log2) << log2, DENOMINATOR, "Denominator must be a power of 2.");
        
        Self(n)
    }
    
    pub fn integral(&self) -> u32 {
        self.0 >> Self::POINT
    }
    
    pub fn fractional(&self) -> (u32, u32) {
        let n = self.0 & !(u32::MAX << Self::POINT);
        let log2gcd = n.trailing_zeros().min(DENOMINATOR.trailing_zeros());
        (n.checked_shr(log2gcd).unwrap_or(0), (DENOMINATOR as u32).checked_shr(log2gcd).unwrap_or(0))
    }
    
    fn to_denominator<const M: usize>(&self) -> FixedPoint<M> {
        if DENOMINATOR > M {
            let shift = Self::POINT - FixedPoint::<M>::POINT;
            let roundoff = self.0 >> (shift - 1);
            FixedPoint((self.0 >> shift) + roundoff)
        } else if DENOMINATOR < M {
            let shift = FixedPoint::<M>::POINT - Self::POINT;
            FixedPoint(self.0 << shift)
        } else {
            FixedPoint(self.0)
        }
    }
}

impl<const DENOMINATOR: usize> Debug for FixedPoint<DENOMINATOR> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?},{:?}/{:?}", self.integral(), self.fractional().0, self.fractional().1)
    }
}

impl<const N: usize, T: Into<u32>> From<T> for FixedPoint<N> {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl<const N: usize> From<FixedPoint<N>> for f32 {
    fn from(value: FixedPoint<N>) -> Self {
        let integral = value.integral();
        let (n, d) = value.fractional();
        integral as f32 + (n as f32 / d as f32)
    }
}

impl<const N: usize, const M: usize> Add<FixedPoint<M>> for FixedPoint<N> where
    [(); max(N, M)] : Sized {
    type Output = FixedPoint<{max(N, M)}>;
    
    fn add(self, rhs: FixedPoint<M>) -> Self::Output {
        let a: FixedPoint<{max(N, M)}> = self.to_denominator();
        let b: FixedPoint<{max(N, M)}> = rhs.to_denominator();
        
        FixedPoint(a.0 + b.0)
    }
}

impl<const N: usize, const M: usize> Mul<FixedPoint<M>> for FixedPoint<N> where
    [(); N * M] : Sized {
    type Output = FixedPoint<{N * M}>;

    fn mul(self, rhs: FixedPoint<M>) -> Self::Output {
        FixedPoint(self.0 * rhs.0)
    }
}

impl<const N: usize, const M: usize> Div<FixedPoint<M>> for FixedPoint<N> {
    type Output = FixedPoint<N>;
    
    fn div(self, rhs: FixedPoint<M>) -> Self::Output {
        FixedPoint((self.0 / rhs.0) * M as u32)
    }
}

impl<const N: usize, const M: usize> PartialEq<FixedPoint<M>> for FixedPoint<N> {
    fn eq(&self, other: &FixedPoint<M>) -> bool {
        if N == M {
            self.0 == other.0
        } else {
            self.integral() == other.integral() && self.fractional() == other.fractional()
        }
    }
}

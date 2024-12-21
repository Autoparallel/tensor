use core::ops::Div;

use super::*;

pub trait Module:
    Add<Output = Self> + Neg<Output = Self> + Mul<Self::Ring, Output = Self> + Copy
{
    type Ring: Add + Neg + Mul + Default + Copy;
}

pub trait VectorSpace: Module
where
    Self::Ring: Div,
{
}

#[derive(Copy, Clone, Debug)]
pub struct Vector<const M: usize, F>(pub [F; M]);

impl<const M: usize, F> Default for Vector<M, F>
where
    F: Default + Copy,
{
    fn default() -> Self {
        Self([F::default(); M])
    }
}

impl<const M: usize, F: Add<Output = F> + Default + Copy> Add for Vector<M, F> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        let mut sum = Self::default();
        for i in 0..M {
            sum.0[i] = self.0[i] + other.0[i];
        }
        sum
    }
}

impl<const M: usize, F: Neg<Output = F> + Default + Copy> Neg for Vector<M, F> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let mut neg = Self::default();
        for i in 0..M {
            neg.0[i] = -self.0[i];
        }
        neg
    }
}

impl<const M: usize, F: Mul<Output = F> + Default + Copy> Mul<F> for Vector<M, F> {
    type Output = Self;
    fn mul(self, scalar: F) -> Self::Output {
        let mut scalar_multiple = Self::default();
        for i in 0..M {
            scalar_multiple.0[i] = scalar * self.0[i];
        }
        scalar_multiple
    }
}

impl<const M: usize, F: Add<Output = F> + Neg<Output = F> + Mul<Output = F> + Default + Copy> Module
    for Vector<M, F>
{
    type Ring = F;
}

impl<
        const M: usize,
        F: Add<Output = F> + Neg<Output = F> + Mul<Output = F> + Div + Default + Copy,
    > VectorSpace for Vector<M, F>
{
}

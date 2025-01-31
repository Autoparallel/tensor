use core::{
    mem::MaybeUninit,
    ops::{Div, Neg},
};

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
pub struct Vector<const M: usize, F: Copy>(pub MaybeUninit<[F; M]>);

// TODO: This could be const.
impl<const M: usize, F> Default for Vector<M, F>
where
    F: Default + Copy,
{
    fn default() -> Self {
        if M == 0 {
            Self(MaybeUninit::uninit())
        } else {
            Self(MaybeUninit::new([F::default(); M]))
        }
    }
}

// TODO: Is it possible to have a compile-time known branch here so we don't
// check the const conditional `M==0` at runtime?
impl<const M: usize, F> Add for Vector<M, F>
where
    F: Add<Output = F> + Default + Copy,
{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        if M == 0 {
            Self(MaybeUninit::uninit())
        } else {
            let mut sum = Self::default();
            let self_arr = unsafe { self.0.assume_init_ref() };
            let other_arr = unsafe { other.0.assume_init_ref() };
            let sum_arr = unsafe { sum.0.assume_init_mut() };

            for i in 0..M {
                sum_arr[i] = self_arr[i] + other_arr[i];
            }
            sum
        }
    }
}

// TODO: Is it possible to have a compile-time known branch here so we don't
// check the const conditional `M==0` at runtime?
impl<const M: usize, F> Neg for Vector<M, F>
where
    F: Neg<Output = F> + Default + Copy,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        if M == 0 {
            Self(MaybeUninit::uninit())
        } else {
            let mut neg = Self::default();
            let self_arr = unsafe { self.0.assume_init_ref() };
            let neg_arr = unsafe { neg.0.assume_init_mut() };

            for i in 0..M {
                neg_arr[i] = -self_arr[i];
            }
            neg
        }
    }
}

impl<const M: usize, F, Inner> Mul<F> for Vector<M, Inner>
where
    F: Mul<Output = F> + Copy + Default,
    Inner: Mul<F, Output = Inner> + Default + Copy,
    [(); M - 1]:, // M > 0
{
    type Output = Self;

    fn mul(self, scalar: F) -> Self::Output {
        let mut scalar_multiple = Self::default();
        let self_arr = self.as_array();
        let result_arr = scalar_multiple.as_array_mut();

        for i in 0..M {
            result_arr[i] = self_arr[i] * scalar;
        }
        scalar_multiple
    }
}

impl<const M: usize, F> Module for Vector<M, F>
where
    F: Add<Output = F> + Neg<Output = F> + Mul<Output = F> + Default + Copy,
{
    type Ring = F;
}

impl<const M: usize, F> VectorSpace for Vector<M, F> where
    F: Add<Output = F> + Neg<Output = F> + Mul<Output = F> + Div + Default + Copy
{
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_dimensional() {
        let nil = Vector::<0, f64>(MaybeUninit::uninit());
        let nil_clone = nil.clone();
        let still_nil = nil + nil_clone;
    }
}

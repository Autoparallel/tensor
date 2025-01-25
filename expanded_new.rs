#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
use core::{fmt::Debug, ops::{Add, Mul}};
pub mod module {
    use core::ops::{Div, Neg};
    use super::*;
    pub trait Module: Add<
            Output = Self,
        > + Neg<Output = Self> + Mul<Self::Ring, Output = Self> + Copy {
        type Ring: Add + Neg + Mul + Default + Copy;
    }
    pub trait VectorSpace: Module
    where
        Self::Ring: Div,
    {}
    pub struct Vector<const M: usize, F>(pub [F; M]);
    #[automatically_derived]
    impl<const M: usize, F: ::core::marker::Copy> ::core::marker::Copy for Vector<M, F> {}
    #[automatically_derived]
    impl<const M: usize, F: ::core::clone::Clone> ::core::clone::Clone for Vector<M, F> {
        #[inline]
        fn clone(&self) -> Vector<M, F> {
            Vector(::core::clone::Clone::clone(&self.0))
        }
    }
    #[automatically_derived]
    impl<const M: usize, F: ::core::fmt::Debug> ::core::fmt::Debug for Vector<M, F> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Vector", &&self.0)
        }
    }
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
    impl<
        const M: usize,
        F: Add<Output = F> + Neg<Output = F> + Mul<Output = F> + Default + Copy,
    > Module for Vector<M, F> {
        type Ring = F;
    }
    impl<
        const M: usize,
        F: Add<Output = F> + Neg<Output = F> + Mul<Output = F> + Div + Default + Copy,
    > VectorSpace for Vector<M, F> {}
}
pub mod tensor {
    use core::ops::AddAssign;
    use super::*;
    pub mod macros {
        use module::Vector;
        use super::*;
        use extensor_macros::tensor;
    }
}
pub use extensor_macros::MultilinearMap;

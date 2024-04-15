#![allow(unstable_features)]
#![feature(generic_const_exprs)]

use std::ops::{Add, Mul};

pub mod coproduct;
pub mod product;
pub mod tensor;

#[derive(Copy, Clone)]
pub struct V<const M: usize, F>([F; M]);

impl<const M: usize, F: Add<Output = F> + Default + Copy> Add for V<M, F> {
    type Output = Self;
    fn add(self, other: V<M, F>) -> Self::Output {
        let mut sum = [F::default(); M];
        for i in 0..M {
            sum[i] = self.0[i] + other.0[i];
        }
        V(sum)
    }
}

impl<const M: usize, F: Mul<Output = F> + Default + Copy> Mul<F> for V<M, F> {
    type Output = Self;
    fn mul(self, scalar: F) -> Self::Output {
        let mut product = [F::default(); M];
        for i in 0..M {
            product[i] = self.0[i] * scalar;
        }
        V(product)
    }
}

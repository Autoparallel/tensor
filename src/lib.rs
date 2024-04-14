#![allow(unstable_features)]
#![feature(generic_const_exprs)]

use std::ops::{Add, Mul};

pub mod coproduct;
pub mod product;
pub mod tensor;

pub struct V<const M: usize>([f64; M]);

impl<const M: usize> Add for V<M> {
    type Output = Self;
    fn add(self, other: V<M>) -> Self::Output {
        let mut sum = [0.0; M];
        for i in 0..M {
            sum[i] = self.0[i] + other.0[i];
        }
        V(sum)
    }
}

impl<const M: usize> Mul<f64> for V<M> {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self::Output {
        let mut product = [0.0; M];
        for i in 0..M {
            product[i] = self.0[i] * scalar;
        }
        V(product)
    }
}

#![allow(unstable_features)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use std::ops::{Add, Mul};

use coproduct::{Coproduct, DirectSum};
use product::{DirectProduct, ProductType};

pub mod coproduct;
pub mod product;
pub mod tensor;
pub mod unique_coproduct;

#[derive(Copy, Clone)]
pub struct V<const M: usize, F>([F; M]);

impl<const M: usize, F> Default for V<M, F>
where
    F: Default + Copy,
{
    fn default() -> Self {
        V([F::default(); M])
    }
}

impl<const M: usize, F: Add<Output = F> + Default + Copy> Add for V<M, F> {
    type Output = Self;
    fn add(self, other: V<M, F>) -> Self::Output {
        let mut sum = V::default();
        for i in 0..M {
            sum.0[i] = self.0[i] + other.0[i];
        }
        sum
    }
}

impl<const M: usize, F: Mul<Output = F> + Default + Copy> Mul<F> for V<M, F> {
    type Output = Self;
    fn mul(self, scalar: F) -> Self::Output {
        let mut scalar_multiple = V::default();
        for i in 0..M {
            scalar_multiple.0[i] = scalar * self.0[i];
        }
        scalar_multiple
    }
}

impl<const M: usize, const N: usize, F> From<DirectSum<M, N, F>> for DirectProduct<M, N, F>
where
    F: Add<Output = F> + Default + Copy,
{
    fn from(sum: DirectSum<M, N, F>) -> DirectProduct<M, N, F> {
        DirectProduct::construct(
            sum.get_X_via_tag().unwrap_or_default(),
            sum.get_Y_via_tag().unwrap_or_default(),
        )
    }
}

impl<const M: usize, const N: usize, F> From<DirectProduct<M, N, F>> for DirectSum<M, N, F>
where
    F: Add<Output = F> + Default + Copy,
{
    fn from(prod: DirectProduct<M, N, F>) -> DirectSum<M, N, F> {
        DirectSum::iota_X(Some(prod.pi_X())) + DirectSum::iota_Y(Some(prod.pi_Y()))
    }
}

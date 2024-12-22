use core::ops::AddAssign;

use module::{Module, Vector};

use super::*;

pub struct SymmetricIndex<const D: usize>
where
    [(); D * (D + 1) / 2]:,
{
    // Store only upper triangular indices
    // Length is (D * (D + 1)) / 2
    indices: [(usize, usize); (D * (D + 1)) / 2],
}

pub struct QuadraticForm<const D: usize, F> {
    eigenbasis: [Vector<D, F>; D],
    eigenvalues: [F; D],
}

impl<const D: usize, F: Copy + ConstDefault + Mul<Output = F> + AddAssign> QuadraticForm<D, F> {
    pub const fn new_diagonal(eigenvalues: [F; D]) -> Self {
        let mut eigenbasis = [Vector::DEFAULT; D];
        let mut i = 0;
        while i < D {
            let mut j = 0;
            while j < D {
                if i == j {
                    eigenbasis[i].0[i] = eigenvalues[i];
                }
                j += 1;
            }
            i += 1;
        }

        Self {
            eigenbasis,
            eigenvalues,
        }
    }

    // TODO: Make `const` if we ever get a const `Mul`
    pub fn eval(&self, lhs: Vector<D, F>, rhs: Vector<D, F>) -> F {
        let mut sum = F::DEFAULT;
        let mut i = 0;
        while i < D {
            // Project vectors onto eigenbasis
            let mut lhs_comp = F::DEFAULT;
            let mut rhs_comp = F::DEFAULT;
            let mut j = 0;
            while j < D {
                lhs_comp += lhs.0[j] * self.eigenbasis[i].0[j];
                rhs_comp += rhs.0[j] * self.eigenbasis[i].0[j];
                j += 1;
            }

            // Multiply by eigenvalue and add to sum
            sum += lhs_comp * rhs_comp * self.eigenvalues[i];
            i += 1;
        }
        sum
    }
}

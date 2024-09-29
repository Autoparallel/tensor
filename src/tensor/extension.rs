//! Something cool to do here with recursion

use core::{
    marker::PhantomData,
    ops::{Add, Mul},
};

use super::{Tensor, V};

trait TensorProduct<F> {
    type T1: TensorProduct<F>;
    type T2: TensorProduct<F>;

    fn tensor_product(tensor_1: Self::T1, tensor_2: Self::T2) -> Self;

    fn multilinear_t1(&self, tensor_1: Self::T1) -> Self::T2;

    fn multilinear_t2(&self, tensor_2: Self::T2) -> Self::T1;
}

impl<const M: usize, F: Default + Add<Output = F> + Mul<Output = F> + Copy> TensorProduct<F>
    for V<M, F>
{
    type T1 = Self;
    type T2 = V<1, F>; // Scalar ring

    fn tensor_product(tensor_1: Self::T1, _tensor_2: Self::T2) -> Self {
        tensor_1
    }

    fn multilinear_t1(&self, tensor_1: Self::T1) -> Self::T2 {
        let val = self
            .0
            .iter()
            .zip(tensor_1.0.iter())
            .fold(F::default(), |acc, (a, b)| acc + (*a * *b));
        V([val])
    }

    fn multilinear_t2(&self, tensor_2: Self::T2) -> Self::T1 {
        *self * tensor_2.0[0]
    }
}

impl<const M: usize, const N: usize, F: Default + Add<Output = F> + Mul<Output = F> + Copy>
    TensorProduct<F> for Tensor<M, N, F>
where
    [(); M * N]:,
{
    type T1 = V<M, F>;
    type T2 = V<N, F>;

    fn tensor_product(tensor_1: Self::T1, tensor_2: Self::T2) -> Self {
        todo!()
    }

    fn multilinear_t1(&self, tensor_1: Self::T1) -> Self::T2 {
        todo!()
    }

    fn multilinear_t2(&self, tensor_2: Self::T2) -> Self::T1 {
        todo!()
    }
}

#[derive(Clone)]
pub struct HigherTensor<T1: TensorProduct<F>, T2: TensorProduct<F>, F> {
    tensor_1: T1,
    tensor_2: T2,
    _p: PhantomData<F>,
}

impl<T1: TensorProduct<F>, T2: TensorProduct<F>, F> TensorProduct<F> for HigherTensor<T1, T2, F> {
    type T1 = T1;
    type T2 = T2;

    fn tensor_product(tensor_1: Self::T1, tensor_2: Self::T2) -> Self {
        todo!()
    }

    fn multilinear_t1(&self, tensor_1: Self::T1) -> Self::T2 {
        todo!()
    }

    fn multilinear_t2(&self, tensor_2: Self::T2) -> Self::T1 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intro() {
        let tensor_1 = Tensor::<3, 2, f64> {
            coefficients: V::<2>(V::<3>([1, 2, 3])),
        };
        let tensor_2 = tensor_1.clone();

        let tensor = HigherTensor {
            tensor_1,
            tensor_2,
            _p: PhantomData,
        };

        let nested_tensor = HigherTensor {
            tensor_1: tensor.clone(),
            tensor_2: tensor.clone(),
            _p: PhantomData,
        };
    }
}

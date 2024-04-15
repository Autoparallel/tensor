use std::ops::AddAssign;

use super::*;

struct Tensor<const M: usize, const N: usize, F>
where
    [(); M * N]:,
{
    /// This set up makes the first index into `coefficients` the "rows" and second the "columns"
    coefficients: V<M, V<N, F>>,
}

impl<const M: usize, const N: usize, F> Default for Tensor<M, N, F>
where
    [(); M * N]:,
    F: Default + Copy,
{
    fn default() -> Self {
        let mut rows = V([F::default(); N]);
        let mut coefficients = V([rows; M]);
        Tensor { coefficients }
    }
}

impl<const M: usize, const N: usize, F> Tensor<M, N, F>
where
    [(); M * N]:,
    F: Mul<Output = F> + Default + Copy,
{
    fn tensor_product<const P: usize>(v: [V<M, F>; P], w: [V<N, F>; P]) -> Tensor<M, N, F> {
        let mut tensor = Tensor::default();

        for p in 0..P {
            for i in 0..M {
                for j in 0..N {
                    tensor.coefficients.0[i].0[j] = v[p].0[i] * w[p].0[j];
                }
            }
        }
        tensor
    }
}

impl<const M: usize, const N: usize, F> Add for Tensor<M, N, F>
where
    [(); M * N]:,
    F: Add<Output = F> + Copy + Default,
{
    type Output = Self;
    fn add(self, other: Tensor<M, N, F>) -> Self::Output {
        let mut tensor = Tensor::default();
        for i in 0..M {
            for j in 0..N {
                tensor.coefficients.0[i].0[j] =
                    self.coefficients.0[i].0[j] + other.coefficients.0[i].0[j];
            }
        }
        tensor
    }
}

impl<const M: usize, const N: usize, F> Mul<F> for Tensor<M, N, F>
where
    [(); M * N]:,
    F: Mul<Output = F> + Default + Copy,
{
    type Output = Self;
    fn mul(self, scalar: F) -> Self::Output {
        let mut tensor = Tensor::default();
        for i in 0..M {
            for j in 0..N {
                tensor.coefficients.0[i].0[j] = self.coefficients.0[i].0[j] * scalar;
            }
        }
        tensor
    }
}

/// Below are more features of tensor that we can define for free!

impl<const M: usize, const N: usize, F> Tensor<M, N, F>
where
    [(); M * N]:,
    F: Add<Output = F> + Mul<Output = F> + AddAssign + Default + Copy,
{
    fn bilinear_map(&self, v: V<M, F>, w: V<N, F>) -> F {
        let mut sum = F::default();
        for i in 0..M {
            for j in 0..N {
                sum += v.0[j] * self.coefficients.0[i].0[j] * w.0[i];
            }
        }
        sum
    }

    /// Here, for each choice of `w`, we get a distinct linear functional on `V` that utilizes the tensor product.
    #[allow(non_snake_case)]
    fn get_functional_on_V(&self, w: V<N, F>) -> impl Fn(V<M, F>) -> F + '_ {
        move |v| self.bilinear_map(v, w)
    }

    /// Here, for each choice of `v`, we get a distinct linear functional on `W` that utilizes the tensor product.
    #[allow(non_snake_case)]
    fn get_functional_on_W(&self, v: V<M, F>) -> impl Fn(V<N, F>) -> F + '_ {
        move |w| self.bilinear_map(v, w)
    }

    /// Matrix multiplication acting from the left :)
    #[allow(non_snake_case)]
    fn linear_map_V_to_W(&self, v: V<M, F>) -> V<N, F> {
        let mut w = V([F::default(); N]);
        for j in 0..N {
            for i in 0..M {
                w.0[j] += self.coefficients.0[i].0[j] * v.0[j];
            }
        }
        w
    }

    /// Matrix multiplication acting from the right :)
    #[allow(non_snake_case)]
    fn linear_map_W_to_V(&self, w: V<N, F>) -> V<M, F> {
        let mut v = V([F::default(); M]);
        for j in 0..N {
            for i in 0..M {
                v.0[j] += self.coefficients.0[i].0[j] * w.0[i];
            }
        }
        v
    }
}

/// This implementation makes `Tensor` an "Algebra" :)
/// In other words, we can multiply M x N matrices with N x P matrices to get an M x P matrix.
impl<const M: usize, const N: usize, const P: usize, F> Mul<Tensor<N, P, F>> for Tensor<M, N, F>
where
    [(); M * N]:,
    [(); N * P]:,
    F: Add<Output = F> + AddAssign + Mul<Output = F> + Default + Copy,
{
    type Output = Self;
    fn mul(self, other: Tensor<N, P, F>) -> Self::Output {
        let mut product = Tensor::default();
        for i in 0..N {
            for k in 0..P {
                for j in 0..M {
                    product.coefficients.0[j].0[k] +=
                        self.coefficients.0[i].0[j] * other.coefficients.0[j].0[k];
                }
            }
        }
        product
    }
}

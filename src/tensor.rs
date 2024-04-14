use super::*;

struct Tensor<const M: usize, const N: usize>
where
    [(); M * N]:,
{
    coefficients: [f64; M * N],
}

impl<const M: usize, const N: usize> Tensor<M, N>
where
    [(); M * N]:,
{
    fn tensor_product<const P: usize>(v: [V<M>; P], w: [V<N>; P]) -> Tensor<M, N> {
        let mut coefficients = [0.0; M * N];

        for p in 0..P {
            for i in 0..M {
                for j in 0..N {
                    coefficients[i * N + j] = v[p].0[i] * w[p].0[j];
                }
            }
        }
        Tensor { coefficients }
    }
}

impl<const M: usize, const N: usize> Add for Tensor<M, N>
where
    [(); M * N]:,
{
    type Output = Self;
    fn add(self, other: Tensor<M, N>) -> Self::Output {
        let mut sum = Tensor {
            coefficients: [0.0; M * N],
        };
        for i in 0..M * N {
            sum.coefficients[i] = self.coefficients[i] + other.coefficients[i];
        }
        sum
    }
}

impl<const M: usize, const N: usize> Mul<f64> for Tensor<M, N>
where
    [(); M * N]:,
{
    type Output = Self;
    fn mul(self, scalar: f64) -> Self::Output {
        let mut product = Tensor {
            coefficients: [0.0; M * N],
        };
        for i in 0..M * N {
            product.coefficients[i] = self.coefficients[i] * scalar;
        }
        product
    }
}

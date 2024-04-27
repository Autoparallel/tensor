use std::ops::AddAssign;

use super::*;

pub struct Tensor<const M: usize, const N: usize, F>
where
    [(); M * N]:,
{
    /// This set up makes the first index into `coefficients` the "rows" and
    /// second the "columns"
    coefficients: V<M, V<N, F>>,
}

impl<const M: usize, const N: usize, F> Default for Tensor<M, N, F>
where
    [(); M * N]:,
    F: Default + Copy,
{
    fn default() -> Self {
        let coefficients = V::<M, V<N, F>>::default();
        Tensor { coefficients }
    }
}

impl<const M: usize, const N: usize, F> Tensor<M, N, F>
where
    [(); M * N]:,
    F: Mul<Output = F> + Default + Copy,
{
    pub fn tensor_product<const P: usize>(v: [V<M, F>; P], w: [V<N, F>; P]) -> Tensor<M, N, F> {
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
    pub fn bilinear_map(&self, v: V<M, F>, w: V<N, F>) -> F {
        let mut sum = F::default();
        for i in 0..M {
            for j in 0..N {
                sum += v.0[j] * self.coefficients.0[i].0[j] * w.0[i];
            }
        }
        sum
    }

    /// Here, for each choice of `w`, we get a distinct linear functional on `V`
    /// that utilizes the tensor product.
    #[allow(non_snake_case)]
    pub fn get_functional_on_V(&self, w: V<N, F>) -> impl Fn(V<M, F>) -> F + '_ {
        move |v| self.bilinear_map(v, w)
    }

    /// Here, for each choice of `v`, we get a distinct linear functional on `W`
    /// that utilizes the tensor product.
    #[allow(non_snake_case)]
    pub fn get_functional_on_W(&self, v: V<M, F>) -> impl Fn(V<N, F>) -> F + '_ {
        move |w| self.bilinear_map(v, w)
    }

    /// Matrix multiplication acting from the left :)
    #[allow(non_snake_case)]
    pub fn linear_map_V_to_W(&self, v: V<M, F>) -> V<N, F> {
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
    pub fn linear_map_W_to_V(&self, w: V<N, F>) -> V<M, F> {
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
/// In other words, we can multiply M x N matrices with N x P matrices to get an
/// M x P matrix.
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

pub struct Tensor3<const M: usize, const N: usize, const P: usize, F>
where
    [(); M * N * P]:,
{
    /// This set up makes the first index into `coefficients` the "rows" and
    /// second the "columns"
    coefficients: V<M, V<N, V<P, F>>>,
}

impl<const M: usize, const N: usize, const P: usize, F> Default for Tensor3<M, N, P, F>
where
    [(); M * N * P]:,
    F: Default + Copy,
{
    fn default() -> Self {
        let coefficients = V::<M, V<N, V<P, F>>>::default();
        Tensor3 { coefficients }
    }
}

macro_rules! tensor {
    ($name:ident, $($const:ident),+) => {
        pub struct $name<$(const $const: usize),+, F>
        where F: Default +  Copy,
        {
            pub coefficients: coeff_builder!($($const),+; F),
        }

        impl<$(const $const: usize),+, F: Default + Copy> Default for  $name<$($const),+, F> {
            fn default() -> Self {
                let coefficients = <def_builder!($($const),+; F)>::default();
                $name { coefficients }
            }

        }

        impl<$(const $const: usize),+, F> std::fmt::Debug for $name<$($const),+, F>
        where
            F: Default + Copy + std::fmt::Debug,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("coefficients", &self.coefficients)
                    .finish()
            }
        }
    }
}

macro_rules! coeff_builder {
    ($const:ident; $expr:ty) => {
        V<$const, $expr>
    };
    ($const:ident, $($rest:ident),+; $expr:ty) => {
        V<$const, coeff_builder!($($rest),+; $expr)>
    };
}

macro_rules! def_builder {
    ($const:ident; $expr:ty) => {
        V::<$const, $expr>
    };
    ($const:ident, $($rest:ident),+; $expr:ty) => {
        V::<$const, def_builder!($($rest),+; $expr)>
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    tensor!(Tensor2, M, N);
    tensor!(Tensor3, M, N, P);

    #[test]
    fn arbitrary_tensor() {
        let tensor = Tensor2::<2, 3, f64>::default();
        println!("{:?}", tensor.coefficients);

        let tensor = Tensor3::<2, 3, 4, f64>::default();
        println!("{:?}", tensor.coefficients);
    }
}

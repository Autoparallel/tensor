use module::Vector;

use super::*;

// TODO: Could probably just assign a valence to the tensors and use N0, N1, N2,
// etc. as dims

#[macro_export]
macro_rules! tensor {
    ($name:ident, $($consts:ident),+) => {
        #[derive(MultilinearMap)]
        pub struct $name<$(const $consts: usize),+, F>
        where F: Default + Copy + AddAssign + Mul<F, Output = F>,
        {
            pub coefficients: coeff_builder!($($consts),+; F),
        }

        impl<$(const $consts: usize),+, F: Default + Copy + AddAssign + Mul<F, Output = F>> Default for  $name<$($consts),+, F> {
            fn default() -> Self {
                let coefficients = <def_builder!($($consts),+; F)>::default();
                $name { coefficients }
            }

        }

        impl<$(const $consts: usize),+, F> Debug for $name<$($consts),+, F>
        where
            F: Default + Copy + Debug + AddAssign + Mul<F, Output = F>,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("coefficients", &self.coefficients)
                    .finish()
            }
        }

        impl<$(const $consts: usize),+, F> Add for $name<$($consts),+, F>
        where
            F: Add<Output = F> + Copy + Default + AddAssign + Mul<F, Output = F>,
        {
            type Output = Self;

            fn add(self, other: Self) -> Self::Output {
                let mut result = Self::default();
                add_tensors!(result.coefficients, self.coefficients, other.coefficients; $($consts),+);
                result
            }
        }

        impl<$(const $consts: usize),+, F> Mul<F> for $name<$($consts),+, F>
        where
            F: Mul<Output = F> + Copy + Default + AddAssign,
        {
            type Output = Self;

            fn mul(self, scalar: F) -> Self::Output {
                let mut result = Self::default();
                scalar_mul_tensor!(result.coefficients, self.coefficients, scalar; $($consts),+);
                result
            }
        }

        impl<$(const $consts: usize),+, F> $name<$($consts),+, F>
        where
            F: Mul<Output = F> + Copy + Default + AddAssign,{
            pub const fn contract<const POS: usize, const DIM: usize,>(&self, v: Vector<DIM, F>) {
                match POS {
                    _ => {}
                }
            }
        }

    }
}

macro_rules! coeff_builder {
    ($const:ident; $expr:ty) => {
        Vector<$const, $expr>
    };
    ($const:ident, $($rest:ident),+; $expr:ty) => {
        Vector<$const, coeff_builder!($($rest),+; $expr)>
    };
}

macro_rules! def_builder {
    ($const:ident; $expr:ty) => {
        Vector::<$const, $expr>
    };
    ($const:ident, $($rest:ident),+; $expr:ty) => {
        Vector::<$const, def_builder!($($rest),+; $expr)>
    };
}

macro_rules! add_tensors {
    ($result:expr, $self:expr, $other:expr; $const:ident) => {
        for i in 0..$const {
            $result.0[i] = $self.0[i] + $other.0[i];
        }
    };
    ($result:expr, $self:expr, $other:expr; $const:ident, $($rest:ident),+) => {
        for i in 0..$const {
            add_tensors!($result.0[i], $self.0[i], $other.0[i]; $($rest),+);
        }
    };
}

macro_rules! scalar_mul_tensor {
    ($result:expr, $self:expr, $scalar:expr; $const:ident) => {
        for i in 0..$const {
            $result.0[i] = $self.0[i] * $scalar;
        }
    };
    ($result:expr, $self:expr, $scalar:expr; $const:ident, $($rest:ident),+) => {
        for i in 0..$const {
            scalar_mul_tensor!($result.0[i], $self.0[i], $scalar; $($rest),+);
        }
    };
}

#[cfg(test)]
mod tests {

    use super::*;
    tensor!(Tensor2, M, N);

    tensor!(Tensor3, M, N, P);

    #[test]
    fn create_arbitrary_tensor() {
        // log();
        let tensor = Tensor2::<2, 3, f64>::default();
        dbg!(tensor.coefficients);

        let tensor = Tensor3::<2, 3, 4, f64>::default();
        dbg!(tensor.coefficients);
    }

    #[test]
    fn add_tensors() {
        // log();
        let mut tensor1 = Tensor2::<2, 3, f64>::default();
        for i in 0..2 {
            for j in 0..3 {
                tensor1.coefficients.0[i].0[j] = (i + j) as f64;
            }
        }
        dbg!(tensor1.coefficients);
        let mut tensor2 = Tensor2::<2, 3, f64>::default();
        for i in 0..2 {
            for j in 0..3 {
                tensor2.coefficients.0[i].0[j] = i as f64 - j as f64;
            }
        }
        dbg!(tensor2.coefficients);
        let tensor3 = tensor1 + tensor2;
        dbg!(tensor3.coefficients);
    }

    #[test]
    fn scalar_mul_tensor() {
        // log();
        let mut tensor1 = Tensor2::<2, 3, f64>::default();
        for i in 0..2 {
            for j in 0..3 {
                tensor1.coefficients.0[i].0[j] = (i + j) as f64;
            }
        }
        dbg!(tensor1.coefficients);
        let scalar = 2.0;
        let tensor2 = tensor1 * scalar;
        dbg!(tensor2.coefficients);
    }

    #[test]
    fn multilinear_map() {
        //           / 1    0     0 \
        // tensor =  \ 0    1     0 /
        let mut tensor = Tensor2::<2, 3, f64>::default();
        tensor.coefficients.0[0].0[0] = 1.0;
        tensor.coefficients.0[1].0[1] = 1.0;
        dbg!(&tensor);

        //        / -1 \
        // v_0 =  \  1 /
        let mut v_0 = Vector::<2, _>::default();
        v_0.0[0] = -1.0;
        v_0.0[1] = 1.0;
        dbg!(v_0);

        //        / 1 \
        //       |  2  |
        // v_1 =  \ 3 /
        let mut v_1 = Vector::<3, _>::default();
        v_1.0[0] = 1.0;
        v_1.0[1] = 2.0;
        v_1.0[2] = 3.0;
        dbg!(v_1);

        //                      / 1 \
        // tensor.map(_,v_1) =  \ 2 /
        //
        // then the next is:
        //                                     / 1 \
        // tensor.map(v_0, v_1) = < -1    1 >  \ 2 /   = -1 + 2 = 1
        let output = tensor.multilinear_map(v_0, v_1);
        dbg!(output);
        assert_eq!(output, 1.0);
    }

    #[test]
    fn test_contraction() {
        let mut tensor = Tensor3::<2, 3, 4, f64>::default();
        // Fill tensor with some values...

        let v = Vector::<2, f64>::default();
        // Contract along M dimension
        let contracted = tensor.contract::<0, 2>(v);

        // contracted is now a Tensor3WithoutM<3,4,f64>
    }
}

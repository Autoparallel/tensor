use super::*;

// TODO: Could probably just assign a valence to the tensors and use N0, N1, N2,
// etc. as dims

#[macro_export]
macro_rules! tensor {
    ($name:ident, $($consts:ident),+) => {
        #[derive(tensor_macros::MultilinearMap)]
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
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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

tensor!(TensorTester, M, N, P);

#[cfg(test)]
mod tests {

    use super::*;
    tensor!(Tensor2, M, N);

    tensor!(Tensor3, M, N, P);

    use log::{debug, info};

    fn log() {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();
    }

    #[test]
    fn create_arbitrary_tensor() {
        log();
        let tensor = Tensor2::<2, 3, f64>::default();
        debug!("{:?}", tensor.coefficients);

        let tensor = Tensor3::<2, 3, 4, f64>::default();
        debug!("{:?}", tensor.coefficients);
    }

    #[test]
    fn add_tensors() {
        log();
        let mut tensor1 = Tensor2::<2, 3, f64>::default();
        for i in 0..2 {
            for j in 0..3 {
                tensor1.coefficients.0[i].0[j] = (i + j) as f64;
            }
        }
        debug!("tensor1: {:?}", tensor1.coefficients);
        let mut tensor2 = Tensor2::<2, 3, f64>::default();
        for i in 0..2 {
            for j in 0..3 {
                tensor2.coefficients.0[i].0[j] = i as f64 - j as f64;
            }
        }
        debug!("tensor2: {:?}", tensor2.coefficients);
        let tensor3 = tensor1 + tensor2;
        info!("output: {:?}", tensor3.coefficients);
    }

    #[test]
    fn scalar_mul_tensor() {
        log();
        let mut tensor1 = Tensor2::<2, 3, f64>::default();
        for i in 0..2 {
            for j in 0..3 {
                tensor1.coefficients.0[i].0[j] = (i + j) as f64;
            }
        }
        debug!("tensor1: {:?}", tensor1.coefficients);
        let scalar = 2.0;
        let tensor2 = tensor1 * scalar;
        info!("output: {:?}", tensor2.coefficients);
    }

    #[test]
    fn multilinear_map() {
        log();
        //           / 1    0     0 \
        // tensor =  \ 0    1     0 /
        let mut tensor = Tensor2::<2, 3, f64>::default();
        tensor.coefficients.0[0].0[0] = 1.0;
        tensor.coefficients.0[1].0[1] = 1.0;
        debug!("tensor: {:?}", tensor);

        //        / -1 \
        // v_0 =  \  1 /
        let mut v_0 = V::default();
        v_0.0[0] = -1.0;
        v_0.0[1] = 1.0;
        debug!("v_0: {:?}", v_0);

        //        / 1 \
        //       |  2  |
        // v_1 =  \ 3 /
        let mut v_1 = V::default();
        v_1.0[0] = 1.0;
        v_1.0[1] = 2.0;
        v_1.0[2] = 3.0;
        debug!("v_1: {:?}", v_1);

        //                      / 1 \
        // tensor.map(_,v_1) =  \ 2 /
        //
        // then the next is:
        //                                     / 1 \
        // tensor.map(v_0, v_1) = < -1    1 >  \ 2 /   = -1 + 2 = 1
        let output = tensor.multilinear_map(v_0, v_1);
        info!("output: {:?}", output);
        assert_eq!(output, 1.0);
    }
}

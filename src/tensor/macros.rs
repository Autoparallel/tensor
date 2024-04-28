use super::*;

#[macro_export]
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

        impl<$(const $const: usize),+, F> Debug for $name<$($const),+, F>
        where
            F: Default + Copy + Debug,
        {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                f.debug_struct(stringify!($name))
                    .field("coefficients", &self.coefficients)
                    .finish()
            }
        }

        impl<$(const $const: usize),+, F> Add for $name<$($const),+, F>
        where
            F: Add<Output = F> + Copy + Default,
        {
            type Output = Self;

            fn add(self, other: Self) -> Self::Output {
                let mut result = Self::default();
                add_tensors!(result.coefficients, self.coefficients, other.coefficients; $($const),+);
                result
            }
        }

        impl<$(const $const: usize),+, F> Mul<F> for $name<$($const),+, F>
        where
            F: Mul<Output = F> + Copy + Default,
        {
            type Output = Self;

            fn mul(self, scalar: F) -> Self::Output {
                let mut result = Self::default();
                scalar_mul_tensor!(result.coefficients, self.coefficients, scalar; $($const),+);
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
}

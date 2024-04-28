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

        impl<$(const $const: usize),+, F> std::ops::Add for $name<$($const),+, F>
        where
            F: std::ops::Add<Output = F> + Copy + Default,
        {
            type Output = Self;

            fn add(self, other: Self) -> Self::Output {
                let mut result = Self::default();
                add_tensors!(result.coefficients, self.coefficients, other.coefficients; $($const),+);
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

#[cfg(test)]
mod tests {
    use super::*;
    tensor!(Tensor2, M, N);
    tensor!(Tensor3, M, N, P);

    #[test]
    fn create_arbitrary_tensor() {
        let tensor = Tensor2::<2, 3, f64>::default();
        println!("{:?}", tensor.coefficients);

        let tensor = Tensor3::<2, 3, 4, f64>::default();
        println!("{:?}", tensor.coefficients);
    }

    #[test]
    fn add_tensors() {
        let mut tensor1 = Tensor2::<2, 3, f64>::default();
        for i in 0..2 {
            for j in 0..3 {
                tensor1.coefficients.0[i].0[j] = (i + j) as f64;
            }
        }
        println!("tensor1: {:?}", tensor1.coefficients);
        let mut tensor2 = Tensor2::<2, 3, f64>::default();
        for i in 0..2 {
            for j in 0..3 {
                tensor2.coefficients.0[i].0[j] = (i as f64 - j as f64);
            }
        }
        println!("tensor2: {:?}", tensor2.coefficients);
        let tensor3 = tensor1 + tensor2;
        println!("output: {:?}", tensor3.coefficients);
    }
}

use core::{
    mem::MaybeUninit,
    ops::{Div, Neg},
};

use super::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Scalar<F>(pub F);

impl<F: Add<Output = F>> Add for Scalar<F> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<F: Mul<Output = F>> Mul for Scalar<F> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl<F: Neg<Output = F>> Neg for Scalar<F> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl<F: Div<Output = F>> Div for Scalar<F> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

pub trait Module:
    Add<Output = Self> + Neg<Output = Self> + Mul<Self::Ring, Output = Self> + Copy
{
    type Ring: Add + Neg + Mul + Default + Copy;
}

pub trait VectorSpace: Module
where
    Self::Ring: Div,
{
}

#[derive(Copy, Clone)]
pub struct Vector<const M: usize, F: ScalarProduct + Copy>(pub MaybeUninit<[F; M]>);

impl<const M: usize, F: Copy + ScalarProduct> Vector<M, F> {
    pub const fn new(arr: [F; M]) -> Self {
        if M == 0 {
            Self(MaybeUninit::uninit())
        } else {
            Self(MaybeUninit::new(arr))
        }
    }
}

pub trait ScalarProduct {
    type Inner;
    fn scalar_product(&self, rhs: &Self) -> Self::Inner;
}

impl<const M: usize, T> ScalarProduct for Vector<M, T>
where
    T: ScalarProduct + Copy + Default,
    T::Inner: Add<Output = T::Inner> + Default,
{
    type Inner = T::Inner;

    fn scalar_product(&self, rhs: &Self) -> Self::Inner {
        if M == 0 {
            T::Inner::default()
        } else {
            let mut scalar_product = T::Inner::default();
            let self_arr = unsafe { self.0.assume_init_ref() };
            let rhs_arr = unsafe { rhs.0.assume_init_ref() };

            for i in 0..M {
                scalar_product = scalar_product + self_arr[i].scalar_product(&rhs_arr[i]);
            }
            scalar_product
        }
    }
}

// Base case - for scalar values
impl<F: Copy + Mul<Output = F>> ScalarProduct for Scalar<F> {
    type Inner = Self;

    fn scalar_product(&self, rhs: &Self) -> Self::Inner {
        *self * *rhs // Use the Mul implementation for Scalar
    }
}

impl<const M: usize, F: Debug + Copy + ScalarProduct> Debug for Vector<M, F> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if M == 0 {
            write!(f, "Vector([])")
        } else {
            let arr = unsafe { self.0.assume_init_ref() };
            write!(f, "Vector({arr:?})")
        }
    }
}

// TODO: This could be const.
impl<const M: usize, F: ScalarProduct> Default for Vector<M, F>
where
    F: Default + Copy,
{
    fn default() -> Self {
        if M == 0 {
            Self(MaybeUninit::uninit())
        } else {
            Self(MaybeUninit::new([F::default(); M]))
        }
    }
}

// TODO: Is it possible to have a compile-time known branch here so we don't
// check the const conditional `M==0` at runtime?
impl<const M: usize, F: ScalarProduct> Add for Vector<M, F>
where
    F: Add<Output = F> + Default + Copy,
{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        if M == 0 {
            Self(MaybeUninit::uninit())
        } else {
            let mut sum = Self::default();
            let self_arr = unsafe { self.0.assume_init_ref() };
            let other_arr = unsafe { other.0.assume_init_ref() };
            let sum_arr = unsafe { sum.0.assume_init_mut() };

            for i in 0..M {
                sum_arr[i] = self_arr[i] + other_arr[i];
            }
            sum
        }
    }
}

// TODO: Is it possible to have a compile-time known branch here so we don't
// check the const conditional `M==0` at runtime?
impl<const M: usize, F: ScalarProduct> Neg for Vector<M, F>
where
    F: Neg<Output = F> + Default + Copy,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        if M == 0 {
            Self(MaybeUninit::uninit())
        } else {
            let mut neg = Self::default();
            let self_arr = unsafe { self.0.assume_init_ref() };
            let neg_arr = unsafe { neg.0.assume_init_mut() };

            for i in 0..M {
                neg_arr[i] = -self_arr[i];
            }
            neg
        }
    }
}

impl<const M: usize, F: ScalarProduct, Inner: ScalarProduct> Mul<F> for Vector<M, Inner>
where
    F: Mul<Output = F> + Copy + Default,
    Inner: Mul<F, Output = Inner> + Default + Copy,
{
    type Output = Self;

    fn mul(self, scalar: F) -> Self::Output {
        if M == 0 {
            Self(MaybeUninit::uninit())
        } else {
            let mut scalar_multiple = Self::default();
            let self_arr = unsafe { self.0.assume_init_ref() };
            let scalar_multiple_arr = unsafe { scalar_multiple.0.assume_init_mut() };

            for i in 0..M {
                scalar_multiple_arr[i] = self_arr[i] * scalar;
            }
            scalar_multiple
        }
    }
}

impl<const M: usize, F: ScalarProduct> Module for Vector<M, F>
where
    F: Add<Output = F> + Neg<Output = F> + Mul<Output = F> + Default + Copy,
{
    type Ring = F;
}

impl<const M: usize, F: ScalarProduct> VectorSpace for Vector<M, F> where
    F: Add<Output = F> + Neg<Output = F> + Mul<Output = F> + Div<Output = F> + Default + Copy
{
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_zero_dimensional_operations() {
        let nil = Vector::<0, Scalar<f64>>::default();
        let nil2 = Vector::<0, Scalar<f64>>::default();

        // Test all operations
        let sum = nil + nil2;
        let neg = -nil;
        let scaled = nil * Scalar(2.0);

        assert_eq!(format!("{:?}", sum), "Vector([])");
        assert_eq!(format!("{:?}", neg), "Vector([])");
        assert_eq!(format!("{:?}", scaled), "Vector([])");
    }

    #[rstest]
    #[case::dim_1([Scalar(1.0)], [Scalar(2.0)], [Scalar(3.0)])]
    #[case::dim_2([Scalar(1.0), Scalar(2.0)], [Scalar(3.0), Scalar(4.0)], [Scalar(4.0), Scalar(6.0)])]
    #[case::dim_3([Scalar(1.0), Scalar(2.0), Scalar(3.0)], [Scalar(4.0), Scalar(5.0), Scalar(6.0)], [Scalar(5.0), Scalar(7.0), Scalar(9.0)])]
    fn test_vector_addition<const M: usize>(
        #[case] a: [Scalar<f64>; M],
        #[case] b: [Scalar<f64>; M],
        #[case] expected: [Scalar<f64>; M],
    ) where
        [(); M - 1]:,
    {
        let va = Vector::new(a);
        let vb = Vector::new(b);
        let sum = va + vb;
        assert_eq!(unsafe { sum.0.assume_init_ref() }, &expected);
    }

    #[rstest]
    #[case::dim_1([Scalar(1.0)], [Scalar(-1.0)])]
    #[case::dim_2([Scalar(1.0), Scalar(2.0)], [Scalar(-1.0), Scalar(-2.0)])]
    #[case::dim_3([Scalar(1.0), Scalar(2.0), Scalar(3.0)], [Scalar(-1.0), Scalar(-2.0), Scalar(-3.0)])]
    fn test_vector_negation<const M: usize>(
        #[case] input: [Scalar<f64>; M],
        #[case] expected: [Scalar<f64>; M],
    ) where
        [(); M - 1]:,
    {
        let v = Vector::new(input);
        let neg = -v;
        assert_eq!(unsafe { neg.0.assume_init_ref() }, &expected);
    }

    #[rstest]
    #[case::dim_1([Scalar(1.0)], Scalar(2.0), [Scalar(2.0)])]
    #[case::dim_2([Scalar(1.0), Scalar(2.0)], Scalar(3.0), [Scalar(3.0), Scalar(6.0)])]
    #[case::dim_3([Scalar(1.0), Scalar(2.0), Scalar(3.0)], Scalar(2.0), [Scalar(2.0), Scalar(4.0), Scalar(6.0)])]
    fn test_vector_scalar_multiplication<const M: usize>(
        #[case] input: [Scalar<f64>; M],
        #[case] scalar: Scalar<f64>,
        #[case] expected: [Scalar<f64>; M],
    ) where
        [(); M - 1]:,
    {
        let v = Vector::new(input);
        let scaled = v * scalar;
        assert_eq!(unsafe { scaled.0.assume_init_ref() }, &expected);
    }

    #[test]
    fn test_vector_default() {
        let v0: Vector<0, Scalar<f64>> = Vector::default();
        let v1: Vector<1, Scalar<f64>> = Vector::default();
        let v3: Vector<3, Scalar<f64>> = Vector::default();

        assert_eq!(format!("{:?}", v0), "Vector([])");
        assert_eq!(unsafe { v1.0.assume_init_ref() }, &[Scalar(0.0)]);
        assert_eq!(
            unsafe { v3.0.assume_init_ref() },
            &[Scalar(0.0), Scalar(0.0), Scalar(0.0)]
        );
    }

    #[test]
    fn test_debug_formatting() {
        let v0 = Vector::<0, Scalar<f64>>::default();
        let v1 = Vector::new([Scalar(1.0)]);
        let v2 = Vector::new([Scalar(1.0), Scalar(2.0)]);

        assert_eq!(format!("{:?}", v0), "Vector([])");
        assert_eq!(format!("{:?}", v1), "Vector([Scalar(1.0)])");
        assert_eq!(format!("{:?}", v2), "Vector([Scalar(1.0), Scalar(2.0)])");
    }

    #[test]
    fn test_vector_traits() {
        fn assert_module<T: Module>() {}
        fn assert_vector_space<T: VectorSpace>()
        where
            T::Ring: Div,
        {
        }

        // Test that Vector implements Module and VectorSpace for f64
        assert_module::<Vector<0, Scalar<f64>>>();
        assert_module::<Vector<1, Scalar<f64>>>();
        assert_vector_space::<Vector<0, Scalar<f64>>>();
        assert_vector_space::<Vector<1, Scalar<f64>>>();
    }

    #[test]
    fn test_copy_clone() {
        let v0 = Vector::<0, Scalar<f64>>::default();
        let v1 = Vector::new([Scalar(1.0)]);

        let v0_copied = v0;
        let v1_copied = v1;

        // Test that copies work correctly
        assert_eq!(format!("{:?}", v0), format!("{:?}", v0_copied));
        assert_eq!(unsafe { v1.0.assume_init_ref() }, unsafe {
            v1_copied.0.assume_init_ref()
        });
    }

    #[test]
    fn test_matrix_scalar_product() {
        // Create two 2x2 matrices as Vector<2, Vector<2, Scalar<f64>>>
        let m1 = Vector::new([
            Vector::new([Scalar(1.0), Scalar(2.0)]),
            Vector::new([Scalar(3.0), Scalar(4.0)]),
        ]);

        let m2 = Vector::new([
            Vector::new([Scalar(5.0), Scalar(6.0)]),
            Vector::new([Scalar(7.0), Scalar(8.0)]),
        ]);

        // The scalar product should be:
        // (1*5 + 2*6) + (3*7 + 4*8) = (5 + 12) + (21 + 32) = 17 + 53 = 70
        let result: Scalar<f64> = m1.scalar_product(&m2);
        assert_eq!(result, Scalar(70.0));
    }

    #[test]
    fn test_nested_vector_operations() {
        // Test creation and scalar product of vectors of different sizes
        let v1 = Vector::new([
            Vector::new([Scalar(1.0), Scalar(2.0), Scalar(3.0)]),
            Vector::new([Scalar(4.0), Scalar(5.0), Scalar(6.0)]),
        ]);

        let v2 = Vector::new([
            Vector::new([Scalar(7.0), Scalar(8.0), Scalar(9.0)]),
            Vector::new([Scalar(10.0), Scalar(11.0), Scalar(12.0)]),
        ]);

        // (1*7 + 2*8 + 3*9) + (4*10 + 5*11 + 6*12)
        // = (7 + 16 + 27) + (40 + 55 + 72)
        // = 50 + 167
        // = 217
        let result = v1.scalar_product(&v2);
        assert_eq!(result, Scalar(217.0));
    }
}

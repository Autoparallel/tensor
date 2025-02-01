use core::{
    mem::MaybeUninit,
    ops::{Div, Neg},
};

use super::*;

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
pub struct Vector<const M: usize, F: Copy>(pub MaybeUninit<[F; M]>);

impl<const M: usize, F: Copy> Vector<M, F>
where
    [(); M - 1]:,
{
    pub const fn new(arr: [F; M]) -> Self {
        Self(MaybeUninit::new(arr))
    }
}

impl<const M: usize, F: Debug + Copy> Debug for Vector<M, F> {
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
impl<const M: usize, F> Default for Vector<M, F>
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
impl<const M: usize, F> Add for Vector<M, F>
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
impl<const M: usize, F> Neg for Vector<M, F>
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

impl<const M: usize, F, Inner> Mul<F> for Vector<M, Inner>
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

impl<const M: usize, F> Module for Vector<M, F>
where
    F: Add<Output = F> + Neg<Output = F> + Mul<Output = F> + Default + Copy,
{
    type Ring = F;
}

impl<const M: usize, F> VectorSpace for Vector<M, F> where
    F: Add<Output = F> + Neg<Output = F> + Mul<Output = F> + Div<Output = F> + Default + Copy
{
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_zero_dimensional_operations() {
        let nil = Vector::<0, f64>::default();
        let nil2 = Vector::<0, f64>::default();

        // Test all operations
        let sum = nil + nil2;
        let neg = -nil;
        let scaled = nil * 2.0;

        assert_eq!(format!("{:?}", sum), "Vector([])");
        assert_eq!(format!("{:?}", neg), "Vector([])");
        assert_eq!(format!("{:?}", scaled), "Vector([])");
    }

    #[rstest]
    #[case::dim_1([1.0], [2.0], [3.0])]
    #[case::dim_2([1.0, 2.0], [3.0, 4.0], [4.0, 6.0])]
    #[case::dim_3([1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [5.0, 7.0, 9.0])]
    fn test_vector_addition<const M: usize>(
        #[case] a: [f64; M],
        #[case] b: [f64; M],
        #[case] expected: [f64; M],
    ) where
        [(); M - 1]:,
    {
        let va = Vector::new(a);
        let vb = Vector::new(b);
        let sum = va + vb;
        assert_eq!(unsafe { sum.0.assume_init_ref() }, &expected);
    }

    #[rstest]
    #[case::dim_1([1.0], [-1.0])]
    #[case::dim_2([1.0, 2.0], [-1.0, -2.0])]
    #[case::dim_3([1.0, 2.0, 3.0], [-1.0, -2.0, -3.0])]
    fn test_vector_negation<const M: usize>(#[case] input: [f64; M], #[case] expected: [f64; M])
    where
        [(); M - 1]:,
    {
        let v = Vector::new(input);
        let neg = -v;
        assert_eq!(unsafe { neg.0.assume_init_ref() }, &expected);
    }

    #[rstest]
    #[case::dim_1([1.0], 2.0, [2.0])]
    #[case::dim_2([1.0, 2.0], 3.0, [3.0, 6.0])]
    #[case::dim_3([1.0, 2.0, 3.0], 2.0, [2.0, 4.0, 6.0])]
    fn test_vector_scalar_multiplication<const M: usize>(
        #[case] input: [f64; M],
        #[case] scalar: f64,
        #[case] expected: [f64; M],
    ) where
        [(); M - 1]:,
    {
        let v = Vector::new(input);
        let scaled = v * scalar;
        assert_eq!(unsafe { scaled.0.assume_init_ref() }, &expected);
    }

    #[test]
    fn test_vector_default() {
        let v0: Vector<0, f64> = Vector::default();
        let v1: Vector<1, f64> = Vector::default();
        let v3: Vector<3, f64> = Vector::default();

        assert_eq!(format!("{:?}", v0), "Vector([])");
        assert_eq!(unsafe { v1.0.assume_init_ref() }, &[0.0]);
        assert_eq!(unsafe { v3.0.assume_init_ref() }, &[0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_debug_formatting() {
        let v0 = Vector::<0, f64>::default();
        let v1 = Vector::new([1.0]);
        let v2 = Vector::new([1.0, 2.0]);

        assert_eq!(format!("{:?}", v0), "Vector([])");
        assert_eq!(format!("{:?}", v1), "Vector([1.0])");
        assert_eq!(format!("{:?}", v2), "Vector([1.0, 2.0])");
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
        assert_module::<Vector<0, f64>>();
        assert_module::<Vector<1, f64>>();
        assert_vector_space::<Vector<0, f64>>();
        assert_vector_space::<Vector<1, f64>>();
    }

    #[test]
    fn test_copy_clone() {
        let v0 = Vector::<0, f64>::default();
        let v1 = Vector::new([1.0]);

        let v0_copied = v0;
        let v1_copied = v1;

        // Test that copies work correctly
        assert_eq!(format!("{:?}", v0), format!("{:?}", v0_copied));
        assert_eq!(unsafe { v1.0.assume_init_ref() }, unsafe {
            v1_copied.0.assume_init_ref()
        });
    }
}

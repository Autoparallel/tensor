use super::*;

pub struct Product<X, Y> {
    pub x: X,
    pub y: Y,
}

pub trait ProductType {
    type X;
    type Y;
    type Z;

    #[allow(non_snake_case)]
    fn f_X(z: Self::Z) -> Self::X;

    #[allow(non_snake_case)]
    fn f_Y(z: Self::Z) -> Self::Y;

    fn f(z: Self::Z) -> Product<Self::X, Self::Y> {
        Product {
            x: Self::f_X(z),
            y: Self::f_Y(z),
        }
    }
}

pub struct DirectProduct<const M: usize, const N: usize, F> {
    v: V<M, F>,
    w: V<N, F>,
}

impl<const M: usize, const N: usize, F> DirectProduct<M, N, F> {
    #[allow(non_snake_case)]
    pub fn pi_V(&self) -> V<M, F> {
        self.v
    }

    #[allow(non_snake_case)]
    pub fn pi_W(&self) -> V<N, F> {
        self.w
    }
}

/// The following would be the implementation of the `ProductType` trait for the
/// `DirectProduct` type, but this won't compile due to Rust complaining
/// ```
/// impl<const M: usize, const N: usize, const P: usize> ProductType for DirectProduct<M, N> {
///     type X = V<M>;
///     type Y = V<N>;
///     type Z = V<P>;

///     fn f_X(z: Self::Z) -> Self::X {
///         unimplemented!()
///     }

///     fn f_Y(z: Self::Z) -> Self::Y {
///         unimplemented!()
///     }
/// }
/// ```

impl<const M: usize, const N: usize, F> Add for DirectProduct<M, N, F>
where
    F: Add<Output = F> + Default + Copy,
{
    type Output = Self;
    fn add(self, other: DirectProduct<M, N, F>) -> Self::Output {
        DirectProduct {
            v: self.pi_V() + other.pi_V(),
            w: self.pi_W() + other.pi_W(),
        }
    }
}

impl<const M: usize, const N: usize, F> Mul<F> for DirectProduct<M, N, F>
where
    F: Mul<Output = F> + Default + Copy,
{
    type Output = Self;
    fn mul(self, scalar: F) -> Self::Output {
        DirectProduct {
            v: self.pi_V() * scalar,
            w: self.pi_W() * scalar,
        }
    }
}

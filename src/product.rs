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

impl<const M: usize, const N: usize> DirectProduct<M, N> {
    #[allow(non_snake_case)]
    pub fn pi_V(&self) -> V<M> {
        self.v
    }

    #[allow(non_snake_case)]
    pub fn pi_W(&self) -> V<N> {
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

pub struct DirectProduct<const M: usize, const N: usize> {
    v: V<M>,
    w: V<N>,
}

impl<const M: usize, const N: usize> DirectProduct<M, N> {
    pub fn new(v: V<M>, w: V<N>) -> Self {
        DirectProduct { v, w }
    }
}

impl<const M: usize, const N: usize> Add for DirectProduct<M, N> {
    type Output = Self;
    fn add(self, other: DirectProduct<M, N>) -> Self::Output {
        DirectProduct {
            v: self.pi_V() + other.pi_V(),
            w: self.pi_W() + other.pi_W(),
        }
    }
}

impl<const M: usize, const N: usize> Mul<f64> for DirectProduct<M, N> {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self::Output {
        DirectProduct {
            v: self.pi_V() * scalar,
            w: self.pi_W() * scalar,
        }
    }
}

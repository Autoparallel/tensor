use super::*;

pub enum UniqueCoproduct<X, Y> {
    X(X),
    Y(Y),
}

pub trait UniqueCoproductType {
    type X;
    type Y;
    type Z;

    #[allow(non_snake_case)]
    fn f_X(x: Self::X) -> Self::Z;

    #[allow(non_snake_case)]
    fn f_Y(y: Self::Y) -> Self::Z;

    fn f(p: UniqueCoproduct<Self::X, Self::Y>) -> Self::Z {
        match p {
            UniqueCoproduct::X(x) => Self::f_X(x),
            UniqueCoproduct::Y(y) => Self::f_Y(y),
        }
    }
}

pub enum UniqueDirectSum<const M: usize, const N: usize, F> {
    V(V<M, F>),
    W(V<N, F>),
}

impl<const M: usize, const N: usize, F> UniqueDirectSum<M, N, F> {
    #[allow(non_snake_case)]
    pub fn iota_V(v: V<M, F>) -> UniqueDirectSum<M, N, F> {
        UniqueDirectSum::V(v)
    }

    #[allow(non_snake_case)]
    pub fn iota_W(w: V<N, F>) -> UniqueDirectSum<M, N, F> {
        UniqueDirectSum::W(w)
    }
}

/// The following would be the implementation of the `UniqueCoproductType` trait for the `UniqueDirectSum` type, but this won't compile due to Rust complaining.
/// ```
/// impl<const M: usize, const N: usize, const P: usize, F> UniqueCoproductType for UniqueDirectSum<M, N, F> {
///     type X = V<M, F>;
///     type Y = V<N, F>;
///     type Z = V<P, F>;

///     #[allow(non_snake_case)]
///     fn f_X(x: Self::X) -> Self::Z {
///         unimplemented!()
///     }

///     #[allow(non_snake_case)]
///     fn f_Y(y: Self::Y) -> Self::Z {
///         unimplemented!()
///     }
/// }

impl<const M: usize, const N: usize, F> Add for UniqueDirectSum<M, N, F>
where
    F: Add<Output = F> + Default + Copy,
{
    type Output = Self;
    fn add(self, other: UniqueDirectSum<M, N, F>) -> Self::Output {
        match (self, other) {
            (UniqueDirectSum::V(v), UniqueDirectSum::V(w)) => UniqueDirectSum::V(V::add(v, w)),
            (UniqueDirectSum::W(v), UniqueDirectSum::W(w)) => UniqueDirectSum::W(V::add(v, w)),
            _ => panic!("Cannot add V and W with Rust `UniqueDirectSum`!"),
        }
    }
}

impl<const M: usize, const N: usize, F> Mul<F> for UniqueDirectSum<M, N, F>
where
    F: Mul<Output = F> + Default + Copy,
{
    type Output = Self;
    fn mul(self, scalar: F) -> Self::Output {
        match self {
            UniqueDirectSum::V(v) => UniqueDirectSum::V(v * scalar),
            UniqueDirectSum::W(w) => UniqueDirectSum::W(w * scalar),
        }
    }
}

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

pub enum UniqueDirectSum<const M: usize, const N: usize> {
    V(V<M>),
    W(V<N>),
}

impl<const M: usize, const N: usize> UniqueDirectSum<M, N> {
    #[allow(non_snake_case)]
    pub fn iota_V(v: V<M>) -> UniqueDirectSum<M, N> {
        UniqueDirectSum::V(v)
    }

    #[allow(non_snake_case)]
    pub fn iota_W(w: V<N>) -> UniqueDirectSum<M, N> {
        UniqueDirectSum::W(w)
    }
}

/// The following would be the implementation of the `UniqueCoproductType` trait for the `UniqueDirectSum` type, but this won't compile due to Rust complaining.
/// ```
/// impl<const M: usize, const N: usize, const P: usize> UniqueCoproductType for UniqueDirectSum<M, N> {
///     type X = V<M>;
///     type Y = V<N>;
///     type Z = V<P>;

///     #[allow(non_snake_case)]
///     fn f_X(x: Self::X) -> Self::Z {
///         unimplemented!()
///     }

///     #[allow(non_snake_case)]
///     fn f_Y(y: Self::Y) -> Self::Z {
///         unimplemented!()
///     }
/// }

impl<const M: usize, const N: usize> Add for UniqueDirectSum<M, N> {
    type Output = Self;
    fn add(self, other: UniqueDirectSum<M, N>) -> Self::Output {
        match (self, other) {
            (UniqueDirectSum::V(v), UniqueDirectSum::V(w)) => UniqueDirectSum::V(V::add(v, w)),
            (UniqueDirectSum::W(v), UniqueDirectSum::W(w)) => UniqueDirectSum::W(V::add(v, w)),
            _ => panic!("Cannot add V and W."),
        }
    }
}

impl<const M: usize, const N: usize> Mul<f64> for UniqueDirectSum<M, N> {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self::Output {
        match self {
            UniqueDirectSum::V(v) => UniqueDirectSum::V(v * scalar),
            UniqueDirectSum::W(w) => UniqueDirectSum::W(w * scalar),
        }
    }
}

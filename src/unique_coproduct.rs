use super::*;

pub enum UniqueDirectSum<const M: usize, const N: usize, F> {
    V(V<M, F>),
    W(V<N, F>),
}

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

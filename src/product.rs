use super::*;

pub trait ProductType
where
    Self: Sized,
{
    type X;
    type Y;

    fn construct(x: Self::X, y: Self::Y) -> Self;

    #[allow(non_snake_case)]
    fn pi_X(&self) -> Self::X;

    #[allow(non_snake_case)]
    fn pi_Y(&self) -> Self::Y;

    #[allow(non_snake_case)]
    fn f<Z>(z: &Z, f_X: impl Fn(&Z) -> Self::X, f_Y: impl Fn(&Z) -> Self::Y) -> Self {
        Self::construct(f_X(z), f_Y(z))
    }
}

#[derive(Copy, Clone)]
pub struct DirectProduct<const M: usize, const N: usize, F> {
    v: V<M, F>,
    w: V<N, F>,
}

impl<const M: usize, const N: usize, F> ProductType for DirectProduct<M, N, F>
where
    F: Copy,
{
    type X = V<M, F>;
    type Y = V<N, F>;

    fn construct(v: Self::X, w: Self::Y) -> Self {
        DirectProduct { v, w }
    }

    fn pi_X(&self) -> Self::X {
        self.v
    }

    fn pi_Y(&self) -> Self::Y {
        self.w
    }
}

impl<const M: usize, const N: usize, F> Add for DirectProduct<M, N, F>
where
    F: Add<Output = F> + Default + Copy,
{
    type Output = Self;
    fn add(self, other: DirectProduct<M, N, F>) -> Self::Output {
        DirectProduct::construct(self.pi_X() + other.pi_X(), self.pi_Y() + other.pi_Y())
    }
}

impl<const M: usize, const N: usize, F> Mul<F> for DirectProduct<M, N, F>
where
    F: Mul<Output = F> + Default + Copy,
{
    type Output = Self;
    fn mul(self, scalar: F) -> Self::Output {
        DirectProduct::construct(self.pi_X() * scalar, self.pi_Y() * scalar)
    }
}

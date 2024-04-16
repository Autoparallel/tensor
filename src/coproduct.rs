use super::*;

pub trait Coproduct
where
    Self: Sized,
{
    type X;
    type Y;

    fn construct(x: Option<Self::X>, y: Option<Self::Y>) -> Self;

    #[allow(non_snake_case)]
    fn iota_X(x: Option<Self::X>) -> Self {
        Self::construct(x, None)
    }

    #[allow(non_snake_case)]
    fn iota_Y(y: Option<Self::Y>) -> Self {
        Self::construct(None, y)
    }

    #[allow(non_snake_case)]
    fn get_X_via_tag(&self) -> Option<Self::X>;

    #[allow(non_snake_case)]
    fn get_Y_via_tag(&self) -> Option<Self::Y>;

    #[allow(non_snake_case)]
    fn f<Z: Add<Output = Z>>(
        &self,
        f_X: impl Fn(Option<Self::X>) -> Z,
        f_Y: impl Fn(Option<Self::Y>) -> Z,
    ) -> Z {
        f_X(self.get_X_via_tag()) + f_Y(self.get_Y_via_tag())
    }
}

pub struct DirectSum<const M: usize, const N: usize, F> {
    v: Option<V<M, F>>,
    w: Option<V<N, F>>,
}

impl<const M: usize, const N: usize, F> Coproduct for DirectSum<M, N, F>
where
    F: Copy,
{
    type X = V<M, F>;
    type Y = V<N, F>;

    fn construct(v: Option<Self::X>, w: Option<Self::Y>) -> Self {
        assert!(v.is_some() || w.is_some());
        DirectSum { v, w }
    }

    fn get_X_via_tag(&self) -> Option<Self::X> {
        self.v
    }

    fn get_Y_via_tag(&self) -> Option<Self::Y> {
        self.w
    }
}

impl<const M: usize, const N: usize, F> Add for DirectSum<M, N, F>
where
    F: Add<Output = F> + Default + Copy,
{
    type Output = Self;
    fn add(self, other: DirectSum<M, N, F>) -> Self::Output {
        DirectSum::construct(
            self.v
                .zip(other.v)
                .map(|(v, other_v)| v + other_v)
                .or(self.v)
                .or(other.v),
            self.w
                .zip(other.w)
                .map(|(w, other_w)| w + other_w)
                .or(self.w)
                .or(other.w),
        )
    }
}

impl<const M: usize, const N: usize, F> Mul<F> for DirectSum<M, N, F>
where
    F: Mul<Output = F> + Default + Copy,
{
    type Output = Self;
    fn mul(self, scalar: F) -> Self::Output {
        DirectSum::construct(self.v.map(|v| v * scalar), self.w.map(|w| w * scalar))
    }
}

use core::marker::PhantomData;

use module::{Module, TrivialModule, Vector};

use super::*;

#[derive(Copy, Clone, Debug)]
pub struct Tensor<A, B>
where
    A: Module,
    B: Module<Ring = A::Ring>,
{
    a: A,
    b: B,
}

impl<A, B> Add for Tensor<A, B>
where
    A: Module,
    B: Module<Ring = A::Ring>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
        }
    }
}

impl<A, B> Neg for Tensor<A, B>
where
    A: Module,
    B: Module<Ring = A::Ring>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            a: -self.a,
            b: -self.b,
        }
    }
}

impl<A, B> Mul<<Self as Module>::Ring> for Tensor<A, B>
where
    A: Module,
    B: Module<Ring = A::Ring>,
{
    type Output = Self;

    fn mul(self, rhs: <Self as Module>::Ring) -> Self::Output {
        Self {
            a: self.a * rhs,
            b: self.b * rhs,
        }
    }
}

impl<A, B> Module for Tensor<A, B>
where
    A: Module,
    B: Module<Ring = A::Ring>,
{
    type Ring = A::Ring;
}

impl<A> Tensor<A, TrivialModule<A::Ring>>
where
    A: Module + Copy,
{
    pub const fn append_trivial<B: Module<Ring = A::Ring> + Copy>(self, b: B) -> Tensor<A, B> {
        let a = self.a;
        Tensor { a, b }
    }
}

impl<A, B> Tensor<A, B>
where
    A: Module + Copy,
    B: Module<Ring = A::Ring> + Copy,
{
    pub const fn product(a: A, b: B) -> Self {
        Self { a, b }
    }

    pub const fn append<C: Module<Ring = A::Ring> + Copy>(self, c: C) -> Tensor<A, Tensor<B, C>> {
        let a = self.a;
        let b = self.b;
        let prod = Tensor { a: b, b: c };
        Tensor { a, b: prod }
    }
}

impl<
        const M: usize,
        F: Add<Output = F> + Neg<Output = F> + Mul<Output = F> + Default + ConstDefault + Copy,
    > From<Vector<M, F>> for Tensor<Vector<M, F>, TrivialModule<F>>
{
    fn from(value: Vector<M, F>) -> Self {
        Self {
            a: value,
            b: TrivialModule { _r: PhantomData },
        }
    }
}

#[cfg(test)]
mod tests {
    use module::Vector;

    use super::*;

    #[test]
    fn intro() {
        let a = Vector::<1, f64>::DEFAULT;
        let b = Vector::<2, f64>::DEFAULT;
        let c = Vector::<3, f64>::DEFAULT;
        let tensor = Tensor::product(a, b);
        let tensor = tensor.append(c);

        let a = Vector::<1, f64>::DEFAULT;
        let b = Vector::<2, f64>::DEFAULT;
        let c = Vector::<3, f64>::DEFAULT;
        let tensor2 = Tensor::product(a, b);
        let tensor2 = tensor2.append(c);

        let added = tensor + tensor2;
        dbg!(added);
    }

    #[test]
    fn terminal() {
        let a = Vector::<1, f64>::DEFAULT;
        let b = Vector::<2, f64>::DEFAULT;
        let tensor = Tensor::from(a);
        let tensor = tensor.append_trivial(b);
    }
}

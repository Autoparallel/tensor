use module::Module;

use super::*;

#[derive(Copy, Clone, Debug)]
pub struct TensorProduct<A, B>
where
    A: Module,
    B: Module<Ring = A::Ring>,
{
    a: A,
    b: B,
}

impl<A, B> Add for TensorProduct<A, B>
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

impl<A, B> Neg for TensorProduct<A, B>
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

impl<A, B> Mul<<Self as Module>::Ring> for TensorProduct<A, B>
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

impl<A, B> Module for TensorProduct<A, B>
where
    A: Module,
    B: Module<Ring = A::Ring>,
{
    type Ring = A::Ring;
}

impl<A, B> TensorProduct<A, B>
where
    A: Module + Copy,
    B: Module<Ring = A::Ring> + Copy,
{
    pub const fn new(a: A, b: B) -> Self {
        Self { a, b }
    }

    pub const fn append<C: Module<Ring = A::Ring> + Copy>(
        self,
        c: C,
    ) -> TensorProduct<A, TensorProduct<B, C>> {
        let a = self.a;
        let b = self.b;
        let prod = TensorProduct { a: b, b: c };
        TensorProduct { a, b: prod }
    }
}

#[cfg(test)]
mod tests {
    use module::Vector;

    use super::*;

    #[test]
    fn intro() {
        let a = Vector::<1, f64>::default();
        let b = Vector::<2, f64>::default();
        let c = Vector::<3, f64>::default();
        let tensor = TensorProduct::new(a, b);
        let tensor = tensor.append(c);

        let a = Vector::<1, f64>::default();
        let b = Vector::<2, f64>::default();
        let c = Vector::<3, f64>::default();
        let tensor2 = TensorProduct::new(a, b);
        let tensor2 = tensor2.append(c);

        let added = tensor + tensor2;
        dbg!(added);
    }
}

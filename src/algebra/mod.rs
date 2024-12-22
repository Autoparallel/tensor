use core::marker::PhantomData;

use module::{Module, Vector};
use tensor::Tensor;

use super::*;

pub mod quadratic_form;

// #[derive(Clone, Copy)]
// pub struct TensorAlgebra<M: Module>(PhantomData<M>);

// impl<M:Module> Module for TensorAlgebra<M> {
//     type Ring = M::Ring;
// }

// impl<M:Module> Add for TensorAlgebra<M> {
//     type Output = TensorAlgebra<M>;

//     fn add(self, rhs: Self) -> Self::Output {
//         todo!()
//     }
// }

// impl<M:Module> Neg for TensorAlgebra<M> {
//     type Output = TensorAlgebra<M>;

//     fn neg(self) -> Self::Output {
//         todo!()
//     }
// }

// impl<M:Module> Mul<M::Ring> for TensorAlgebra<M> {
//     type Output = TensorAlgebra<M>;

//     fn mul(self, rhs: M::Ring) -> Self::Output {
//         todo!()
//     }
// }

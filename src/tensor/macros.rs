use module::Vector;

use super::*;

//         impl<$(const $consts: usize),+, F> $name<$($consts),+, F>
//         where
//             F: Mul<Output = F> + Copy + Default + AddAssign,{
//             pub const fn contract<const POS: usize, const DIM: usize,>(&self,
// v: Vector<DIM, F>) {                 match POS {
//                     _ => {}
//                 }
//             }
//         }

//     }
// }

tensor!(3);

impl<
        const N0: usize,
        const N1: usize,
        const N2: usize,
        F: Default + Copy + AddAssign + Mul<F, Output = F>,
    > Tensor<N0, N1, N2, F>
{
    pub fn contract<const SLICE: usize, const DIM: usize>(
        &self,
        vector: Vector<DIM, F>,
    ) -> Tensor<
        { (1 - (SLICE == 0) as usize) * N0 },
        { (1 - (SLICE == 1) as usize) * N1 },
        { (1 - (SLICE == 2) as usize) * N2 },
        F,
    >
    where
        [(); (SLICE < 3) as usize - 1]:,
        [(); (SLICE == 0) as usize * ((DIM == N0) as usize)
            + (SLICE == 1) as usize * ((DIM == N1) as usize)
            + (SLICE == 2) as usize * ((DIM == N2) as usize)
            - 1]:,
    {
        todo!()
    }
}

// pub const fn check_range<const VALENCE: usize, const SLICE: usize, const DIM:
// usize>(     arr: [usize; VALENCE],
// ) -> bool
// where
//     [(); (SLICE < VALENCE) as usize]:,
// {
//     arr[SLICE] == DIM
// }

#[cfg(test)]
mod tests {

    use super::*;

    // tensor!(2);

    // #[test]
    // fn create_arbitrary_tensor() {
    //     let tensor = Tensor::<2, 3, f64>::default();
    //     dbg!(tensor);
    // }

    // #[test]
    // fn add_tensors() {
    //     // log();
    //     let mut tensor1 = Tensor::<2, 3, f64>::default();
    //     for i in 0..2 {
    //         for j in 0..3 {
    //             tensor1.coefficients.0[i].0[j] = (i + j) as f64;
    //         }
    //     }
    //     dbg!(tensor1.coefficients);
    //     let mut tensor2 = Tensor::<2, 3, f64>::default();
    //     for i in 0..2 {
    //         for j in 0..3 {
    //             tensor2.coefficients.0[i].0[j] = i as f64 - j as f64;
    //         }
    //     }
    //     dbg!(tensor2.coefficients);
    //     let tensor3 = tensor1 + tensor2;
    //     dbg!(tensor3.coefficients);
    // }

    // #[test]
    // fn scalar_mul_tensor() {
    //     // log();
    //     let mut tensor1 = Tensor::<2, 3, f64>::default();
    //     for i in 0..2 {
    //         for j in 0..3 {
    //             tensor1.coefficients.0[i].0[j] = (i + j) as f64;
    //         }
    //     }
    //     dbg!(tensor1.coefficients);
    //     let scalar = 2.0;
    //     let tensor2 = tensor1 * scalar;
    //     dbg!(tensor2.coefficients);
    // }

    // #[test]
    // fn multilinear_map() {
    //     //           / 1    0     0 \
    //     // tensor =  \ 0    1     0 /
    //     let mut tensor = Tensor::<2, 3, f64>::default();
    //     tensor.coefficients.0[0].0[0] = 1.0;
    //     tensor.coefficients.0[1].0[1] = 1.0;
    //     dbg!(&tensor);

    //     //        / -1 \
    //     // v_0 =  \  1 /
    //     let mut v_0 = Vector::<2, _>::default();
    //     v_0.0[0] = -1.0;
    //     v_0.0[1] = 1.0;
    //     dbg!(v_0);

    //     //        / 1 \
    //     //       |  2  |
    //     // v_1 =  \ 3 /
    //     let mut v_1 = Vector::<3, _>::default();
    //     v_1.0[0] = 1.0;
    //     v_1.0[1] = 2.0;
    //     v_1.0[2] = 3.0;
    //     dbg!(v_1);

    //     //                      / 1 \
    //     // tensor.map(_,v_1) =  \ 2 /
    //     //
    //     // then the next is:
    //     //                                     / 1 \
    //     // tensor.map(v_0, v_1) = < -1    1 >  \ 2 /   = -1 + 2 = 1
    //     let output = tensor.multilinear_map(v_0, v_1);
    //     dbg!(output);
    //     assert_eq!(output, 1.0);
    // }

    #[test]
    fn test_contraction() {
        let mut tensor = Tensor::<2, 3, 4, f64>::default();

        // Fill tensor with some values...

        let v = Vector::<4, f64>::default();
        // Contract along M dimension
        let contracted: Tensor<2, 3, 0, f64> = tensor.contract::<2, 4>(v);
    }
}

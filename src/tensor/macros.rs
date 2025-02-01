use module::{ScalarProduct, Vector};

use super::*;

tensor!(3);

// impl<
//         const N0: usize,
//         const N1: usize,
//         const N2: usize,
//         F: ScalarProduct
//             + Default
//             + Copy
//             + AddAssign
//             + Mul<F, Output = F>
//             + core::ops::Add<Output = F>,
//     > Tensor<N0, N1, N2, F>
// where
//     F::Inner: Add<Output = F::Inner> + Default + Copy,
// {
//     pub fn contract<const SLICE: usize, const DIM: usize>(
//         &self,
//         vector: Vector<DIM, F>,
//     ) -> Tensor<
//         { (1 - (SLICE == 0) as usize) * N0 },
//         { (1 - (SLICE == 1) as usize) * N1 },
//         { (1 - (SLICE == 2) as usize) * N2 },
//         F,
//     >
//     where
//         [(); (SLICE < 3) as usize - 1]:,
//         [(); (SLICE == 0) as usize * ((DIM == N0) as usize)
//             + (SLICE == 1) as usize * ((DIM == N1) as usize)
//             + (SLICE == 2) as usize * ((DIM == N2) as usize)
//             - 1]:,
//     {
//         let mut result = Tensor::default();

//         match SLICE {
//             0 => {
//                 for i1 in 0..N1 {
//                     for i2 in 0..N2 {
//                         let mut sum = F::default();
//                         for i0 in 0..N0 {
//                             sum = sum + self.coefficients.0[i0].0[i1].0[i2] *
// vector.0[i0];                         }
//                         result.coefficients.0[0].0[i1].0[i2] = sum;
//                     }
//                 }
//             }
//             1 => {
//                 for i0 in 0..N0 {
//                     for i2 in 0..N2 {
//                         let mut sum = F::default();
//                         for i1 in 0..N1 {
//                             sum = sum + self.coefficients.0[i0].0[i1].0[i2] *
// vector.0[i1];                         }
//                         result.coefficients.0[i0].0[0].0[i2] = sum;
//                     }
//                 }
//             }
//             2 => {
//                 for i0 in 0..N0 {
//                     for i1 in 0..N1 {
//                         let mut sum = F::default();
//                         for i2 in 0..N2 {
//                             sum = sum + self.coefficients.0[i0].0[i1].0[i2] *
// vector.0[i2];                         }
//                         result.coefficients.0[i0].0[i1].0[0] = sum;
//                     }
//                 }
//             }
//             _ => unreachable!(), // Our where clause ensures this
//         }

//         result
//     }
// }

// #[cfg(test)]
// mod tests {

//     use super::*;

//     // tensor!(2);

//     // #[test]
//     // fn create_arbitrary_tensor() {
//     //     let tensor = Tensor::<2, 3, f64>::default();
//     //     dbg!(tensor);
//     // }

//     // #[test]
//     // fn add_tensors() {
//     //     // log();
//     //     let mut tensor1 = Tensor::<2, 3, f64>::default();
//     //     for i in 0..2 {
//     //         for j in 0..3 {
//     //             tensor1.coefficients.0[i].0[j] = (i + j) as f64;
//     //         }
//     //     }
//     //     dbg!(tensor1.coefficients);
//     //     let mut tensor2 = Tensor::<2, 3, f64>::default();
//     //     for i in 0..2 {
//     //         for j in 0..3 {
//     //             tensor2.coefficients.0[i].0[j] = i as f64 - j as f64;
//     //         }
//     //     }
//     //     dbg!(tensor2.coefficients);
//     //     let tensor3 = tensor1 + tensor2;
//     //     dbg!(tensor3.coefficients);
//     // }

//     // #[test]
//     // fn scalar_mul_tensor() {
//     //     // log();
//     //     let mut tensor1 = Tensor::<2, 3, f64>::default();
//     //     for i in 0..2 {
//     //         for j in 0..3 {
//     //             tensor1.coefficients.0[i].0[j] = (i + j) as f64;
//     //         }
//     //     }
//     //     dbg!(tensor1.coefficients);
//     //     let scalar = 2.0;
//     //     let tensor2 = tensor1 * scalar;
//     //     dbg!(tensor2.coefficients);
//     // }

//     // #[test]
//     // fn multilinear_map() {
//     //     //           / 1    0     0 \
//     //     // tensor =  \ 0    1     0 /
//     //     let mut tensor = Tensor::<2, 3, f64>::default();
//     //     tensor.coefficients.0[0].0[0] = 1.0;
//     //     tensor.coefficients.0[1].0[1] = 1.0;
//     //     dbg!(&tensor);

//     //     //        / -1 \
//     //     // v_0 =  \  1 /
//     //     let mut v_0 = Vector::<2, _>::default();
//     //     v_0.0[0] = -1.0;
//     //     v_0.0[1] = 1.0;
//     //     dbg!(v_0);

//     //     //        / 1 \
//     //     //       |  2  |
//     //     // v_1 =  \ 3 /
//     //     let mut v_1 = Vector::<3, _>::default();
//     //     v_1.0[0] = 1.0;
//     //     v_1.0[1] = 2.0;
//     //     v_1.0[2] = 3.0;
//     //     dbg!(v_1);

//     //     //                      / 1 \
//     //     // tensor.map(_,v_1) =  \ 2 /
//     //     //
//     //     // then the next is:
//     //     //                                     / 1 \
//     //     // tensor.map(v_0, v_1) = < -1    1 >  \ 2 /   = -1 + 2 = 1
//     //     let output = tensor.multilinear_map(v_0, v_1);
//     //     dbg!(output);
//     //     assert_eq!(output, 1.0);
//     // }

//     #[test]
//     fn test_contraction() {
//         let mut tensor = Tensor::<2, 3, 4, f64>::default();

//         // Fill tensor with some values...

//         let v = Vector::<4, f64>::default();
//         // Contract along M dimension
//         let contracted: Tensor<2, 3, 0, f64> = tensor.contract::<2, 4>(v);
//     }

//     #[test]
//     fn test_rank3_contraction() {
//         // Create a 2x3x2 tensor
//         let mut tensor = Tensor::<2, 3, 2, f64>::default();

//         // Fill tensor with some known values
//         // Using a simple pattern: tensor[i][j][k] = i + j + k
//         for i in 0..2 {
//             for j in 0..3 {
//                 for k in 0..2 {
//                     tensor.coefficients.0[i].0[j].0[k] = (i + j + k) as f64;
//                 }
//             }
//         }

//         // Test contraction along slice 0 (first dimension)
//         let v0 = Vector([1.0, 2.0]); // 2-dimensional vector for N0
//         let contracted0: Tensor<0, 3, 2, f64> = tensor.contract::<0, 2>(v0);
//         // Expected: contracted0[j][k] = sum_i(tensor[i][j][k] * v0[i])

//         // Test contraction along slice 1 (second dimension)
//         let v1 = Vector([1.0, 2.0, 3.0]); // 3-dimensional vector for N1
//         let contracted1: Tensor<2, 0, 2, f64> = tensor.contract::<1, 3>(v1);
//         // Expected: contracted1[i][k] = sum_j(tensor[i][j][k] * v1[j])
//         dbg!(contracted1);

//         // Test contraction along slice 2 (third dimension)
//         let v2 = Vector([1.0, 2.0]); // 2-dimensional vector for N2
//         let contracted2: Tensor<2, 3, 0, f64> = tensor.contract::<2, 2>(v2);
//         // Expected: contracted2[i][j] = sum_k(tensor[i][j][k] * v2[k])

//         // Verify specific values
//         // Let's check one value from each contraction

//         // For slice 0: contracted0[1][1] should be
//         // tensor[0][1][1] * v0[0] + tensor[1][1][1] * v0[1]
//         // assert_eq!(
//         //     contracted0.coefficients.0[0].0[1].0[1],
//         //     (2.0 * 1.0 + 3.0 * 2.0)
//         // );

//         // // For slice 1: contracted1[1][1] should be
//         // // tensor[1][0][1] * v1[0] + tensor[1][1][1] * v1[1] +
//         // tensor[1][2][1] * v1[2] assert_eq!(
//         //     contracted1.coefficients.0[1].0[0].0[1],
//         //     (2.0 * 1.0 + 3.0 * 2.0 + 4.0 * 3.0)
//         // );

//         // // For slice 2: contracted2[1][2] should be
//         // // tensor[1][2][0] * v2[0] + tensor[1][2][1] * v2[1]
//         // assert_eq!(
//         //     contracted2.coefficients.0[1].0[2].0[0],
//         //     (3.0 * 1.0 + 4.0 * 2.0)
//         // );
//     }
// }

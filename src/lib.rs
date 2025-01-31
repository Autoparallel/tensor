#![no_std]
#![feature(generic_const_exprs)]

use core::{
    fmt::Debug,
    ops::{Add, Mul},
};

pub mod module;
pub mod tensor;

pub use extensor_macros::{tensor, MultilinearMap};

#[cfg(test)]
#[macro_use]
extern crate std;

#![allow(unstable_features)]
#![allow(incomplete_features)]
#![feature(specialization)]
#![no_std]

use core::{
    fmt::Debug,
    ops::{Add, Mul, Neg},
};

#[cfg(test)]
#[macro_use]
extern crate std;

pub mod module;
pub mod tensor;

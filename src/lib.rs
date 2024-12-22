#![allow(unstable_features)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]
#![no_std]

use core::{
    fmt::Debug,
    ops::{Add, Mul, Neg},
};

use const_default::ConstDefault;

#[cfg(test)]
#[macro_use]
extern crate std;

pub mod algebra;
pub mod module;
pub mod tensor;

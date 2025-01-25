#![allow(unstable_features)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![no_std]

use core::{
    fmt::{Debug, Formatter, Result},
    ops::{Add, Mul},
};

pub mod module;
pub mod tensor;

#[cfg(test)]
#[macro_use]
extern crate std;

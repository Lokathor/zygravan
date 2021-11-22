#![no_std]
#![feature(asm)]
#![feature(asm_const)]

mod bit_utils;
pub use bit_utils::*;
mod macros;

pub mod gba;

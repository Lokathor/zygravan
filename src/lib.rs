#![no_std]
#![feature(asm)]
#![feature(asm_const)]

mod macros;

mod bit_utils;
pub use bit_utils::*;

pub mod gba;

pub mod voladdress_next;
pub use voladdress_next::*;

pub mod fixed_point;
pub use fixed_point::*;

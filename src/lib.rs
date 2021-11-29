#![no_std]
#![feature(asm)]
#![feature(asm_const)]
#![feature(isa_attribute)]

mod macros;

mod bit_utils;

pub use bit_utils::*;

pub mod gba;
use gba::{a32_swpb, GbaCell};

pub mod fixed_point;
pub use fixed_point::*;

static EWRAM_STATE: GbaCell<u8> = unsafe { GbaCell::new(0) };
pub struct Ewram(());
impl Ewram {
  const EWRAM_BASE: usize = 0x0200_0000;

  pub fn try_new() -> Option<Self> {
    if unsafe { a32_swpb(1, EWRAM_STATE.get_ptr()) } != 0 {
      None
    } else {
      Some(Self(()))
    }
  }
}
impl core::ops::Drop for Ewram {
  fn drop(&mut self) {
    unsafe { a32_swpb(0, EWRAM_STATE.get_ptr()) };
  }
}
impl core::ops::Deref for Ewram {
  type Target = [u32; 65536];
  fn deref(&self) -> &Self::Target {
    unsafe { &*(Self::EWRAM_BASE as *const Self::Target) }
  }
}
impl core::ops::DerefMut for Ewram {
  fn deref_mut(&mut self) -> &mut Self::Target {
    unsafe { &mut *(Self::EWRAM_BASE as *mut Self::Target) }
  }
}

use core::{cell::UnsafeCell, mem::size_of};
use voladdress::{Safe, VolAddress, VolBlock, VolRegion, VolSeries};

use crate::{
  macros::{const_new, u16_bool_field, u16_enum_field, u16_value_field},
  Fx,
};

mod bios;
pub use bios::*;

mod bg_charblock;
pub use bg_charblock::*;

mod default_art;
pub use default_art::*;

mod display_control;
pub use display_control::*;

mod display_status;
pub use display_status::*;

mod key_input;
pub use key_input::*;

mod palette;
pub use palette::*;

mod text_screenblock;
pub use text_screenblock::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct IrqBits(pub(crate) u16);
impl IrqBits {
  const_new!();
  u16_bool_field!(0, vblank, with_vblank);
  u16_bool_field!(1, hblank, with_hblank);
  u16_bool_field!(2, vcounter, with_vcounter);
  u16_bool_field!(3, timer0, with_timer0);
  u16_bool_field!(4, timer1, with_timer1);
  u16_bool_field!(5, timer2, with_timer2);
  u16_bool_field!(6, timer3, with_timer3);
  u16_bool_field!(7, serial, with_serial);
  u16_bool_field!(8, dma0, with_dma0);
  u16_bool_field!(9, dma1, with_dma1);
  u16_bool_field!(10, dma2, with_dma2);
  u16_bool_field!(11, dma3, with_dma3);
  u16_bool_field!(12, keypad, with_keypad);
  u16_bool_field!(13, game_pak, with_game_pak);
}
pub const IE: VolAddress<IrqBits, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_0200) };
pub const IME: VolAddress<bool, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_0208) };

#[derive(Debug)]
#[repr(transparent)]
pub struct GbaCell<T>(UnsafeCell<T>);
unsafe impl<T> Sync for GbaCell<T> {}
impl<T> GbaCell<T> {
  /// Makes a new value
  ///
  /// ## Safety
  /// * You must **only** use this with types that are accessed with a single
  ///   instruction.
  /// * This means just 1, 2, and 4 byte integer values, or newtype wrappers
  ///   over such values.
  /// * Also allowed is pointers (both function and data).
  /// * Do **not** put any multi-field structs in a `GbaCell`
  #[inline]
  #[must_use]
  pub const unsafe fn new(t: T) -> Self {
    Self(UnsafeCell::new(t))
  }
  #[inline]
  #[must_use]
  pub fn read(&self) -> T {
    unsafe { self.0.get().read_volatile() }
  }
  #[inline]
  pub fn write(&self, t: T) {
    unsafe { self.0.get().write_volatile(t) }
  }
  #[inline]
  #[must_use]
  pub fn get_ptr(&self) -> *mut T {
    self.0.get()
  }
}
impl GbaCell<u32> {
  #[inline]
  #[must_use]
  pub const fn new_u32(u: u32) -> Self {
    unsafe { Self::new(u) }
  }
}

pub type RustIrqFn = extern "C" fn(IrqBits);

#[inline(always)]
pub fn set_irq_handler(opt_fn: Option<RustIrqFn>) {
  extern "C" {
    static RUST_IRQ_HANDLER: GbaCell<Option<RustIrqFn>>;
  }
  //
  unsafe { RUST_IRQ_HANDLER.write(opt_fn) }
}

pub const BG0CNT: VolAddress<BgControl, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_0008) };
pub const BG1CNT: VolAddress<BgControl, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_000A) };
pub const BG2CNT: VolAddress<BgControl, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_000C) };
pub const BG3CNT: VolAddress<BgControl, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_000E) };

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct BgControl(u16);
impl BgControl {
  const_new!();
  u16_value_field!(0 - 1, z_index, with_z_index);
  u16_value_field!(2 - 3, charblock, with_charblock);
  u16_bool_field!(6, mosaic, with_mosaic);
  u16_bool_field!(7, is_8bpp, with_8bpp);
  u16_value_field!(8 - 12, screenblock, with_screenblock);
  u16_bool_field!(13, affine_wrap, with_affine_wrap);
  u16_value_field!(14 - 15, screen_size, with_screen_size);
}

/*
Internal Screen Size (dots) and size of BG Map (bytes):
  Value  Text Mode (w,h)   Affine Mode
  0      256x256 (2K)      128x128   (256 bytes)
  1      512x256 (4K)      256x256   (1K)
  2      256x512 (4K)      512x512   (4K)
  3      512x512 (8K)      1024x1024 (16K)
*/

pub type Tile4 = [u32; (4 * 8 * 8) / 32];
pub type Tile8 = [u32; (8 * 8 * 8) / 32];

pub type AffineScreenblockS0 = VolBlock<u8, Safe, Safe, { 16 * 16 }>;
pub type AffineScreenblockS1 = VolBlock<u8, Safe, Safe, { 32 * 32 }>;
pub type AffineScreenblockS2 = VolBlock<u8, Safe, Safe, { 64 * 64 }>;
pub type AffineScreenblockS3 = VolBlock<u8, Safe, Safe, { 128 * 128 }>;

/// `N` in `0..32`
#[inline]
#[must_use]
pub const fn affine_screenblock_s0<const N: usize>() -> AffineScreenblockS0 {
  assert!(N < 32);
  unsafe {
    VolBlock::new(0x0600_0000 + N * size_of::<[TextScreenEntry; 32 * 32]>())
  }
}
/// `N` in `0..32`
#[inline]
#[must_use]
pub const fn affine_screenblock_s1<const N: usize>() -> AffineScreenblockS1 {
  assert!(N < 32);
  unsafe {
    VolBlock::new(0x0600_0000 + N * size_of::<[TextScreenEntry; 32 * 32]>())
  }
}
/// `N` in `0..30`
#[inline]
#[must_use]
pub const fn affine_screenblock_s2<const N: usize>() -> AffineScreenblockS2 {
  assert!(N < 30);
  unsafe {
    VolBlock::new(0x0600_0000 + N * size_of::<[TextScreenEntry; 32 * 32]>())
  }
}
/// `N` in `0..24`
#[inline]
#[must_use]
pub const fn affine_screenblock_s3<const N: usize>() -> AffineScreenblockS3 {
  assert!(N < 24);
  unsafe {
    VolBlock::new(0x0600_0000 + N * size_of::<[TextScreenEntry; 32 * 32]>())
  }
}

pub const BG0_X: VolAddress<u16, (), Safe> =
  unsafe { VolAddress::new(0x0400_0010) };
pub const BG0_Y: VolAddress<u16, (), Safe> =
  unsafe { VolAddress::new(0x0400_0012) };

pub const BG1_X: VolAddress<u16, (), Safe> =
  unsafe { VolAddress::new(0x0400_0014) };
pub const BG1_Y: VolAddress<u16, (), Safe> =
  unsafe { VolAddress::new(0x0400_0016) };

pub const BG2_X: VolAddress<u16, (), Safe> =
  unsafe { VolAddress::new(0x0400_0018) };
pub const BG2_Y: VolAddress<u16, (), Safe> =
  unsafe { VolAddress::new(0x0400_001A) };

pub const BG3_X: VolAddress<u16, (), Safe> =
  unsafe { VolAddress::new(0x0400_001C) };
pub const BG3_Y: VolAddress<u16, (), Safe> =
  unsafe { VolAddress::new(0x0400_001E) };

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ObjMode {
  Normal = (0 << 10),
  SemiTransparent = (1 << 10),
  Window = (2 << 10),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ObjShape {
  Square = (0 << 14),
  Horizontal = (1 << 14),
  Vertical = (2 << 14),
}
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct ObjAttr0(u16);
impl ObjAttr0 {
  const_new!();
  u16_value_field!(0 - 7, y, with_y);
  u16_bool_field!(8, affine, with_affine);
  u16_bool_field!(9, disabled, with_disabled);
  u16_bool_field!(9, affine_double_size, with_affine_double_size);
  u16_enum_field!(10 - 11: ObjMode, mode, with_mode);
  u16_bool_field!(12, mosaic, with_mosaic);
  u16_bool_field!(13, is_8bpp, with_is_8bpp);
  u16_enum_field!(14 - 15: ObjShape, shape, with_shape);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct ObjAttr1(u16);
impl ObjAttr1 {
  const_new!();
  u16_value_field!(0 - 8, x, with_x);
  u16_value_field!(9 - 13, affine_index, with_affine_index);
  u16_bool_field!(12, hflip, with_hflip);
  u16_bool_field!(13, vflip, with_vflip);
  u16_value_field!(14 - 15, obj_size, with_obj_size);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct ObjAttr2(u16);
impl ObjAttr2 {
  const_new!();
  u16_value_field!(0 - 9, base_tile, with_base_tile);
  u16_value_field!(10 - 11, z_index, with_z_index);
  u16_value_field!(12 - 15, palbank, with_palbank);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(C)]
pub struct Obj(ObjAttr0, ObjAttr1, ObjAttr2);

pub const OAM: VolSeries<Obj, Safe, Safe, 128, { size_of::<[u16; 4]>() }> =
  unsafe { VolSeries::new(0x0700_0000) };

#[rustfmt::skip]
pub const OAM0: VolSeries<ObjAttr0, Safe, Safe, 128, { size_of::<[u16; 4]>() }> =
  unsafe { VolSeries::new(0x0700_0000) };
#[rustfmt::skip]
pub const OAM1: VolSeries<ObjAttr1, Safe, Safe, 128, { size_of::<[u16; 4]>() }> =
  unsafe { VolSeries::new(0x0700_0002) };
#[rustfmt::skip]
pub const OAM2: VolSeries<ObjAttr2, Safe, Safe, 128, { size_of::<[u16; 4]>() }> =
  unsafe { VolSeries::new(0x0700_0004) };

#[rustfmt::skip]
pub const PA: VolSeries<Fx<i16,8>, Safe, Safe, 32, { size_of::<[u16; 16]>() }> =
  unsafe { VolSeries::new(0x0700_0006) };
#[rustfmt::skip]
pub const PB: VolSeries<Fx<i16,8>, Safe, Safe, 32, { size_of::<[u16; 16]>() }> =
  unsafe { VolSeries::new(0x0700_000E) };
#[rustfmt::skip]
pub const PC: VolSeries<Fx<i16,8>, Safe, Safe, 32, { size_of::<[u16; 16]>() }> =
  unsafe { VolSeries::new(0x0700_0016) };
#[rustfmt::skip]
pub const PD: VolSeries<Fx<i16,8>, Safe, Safe, 32, { size_of::<[u16; 16]>() }> =
  unsafe { VolSeries::new(0x0700_001E) };

#[inline(never)]
#[link_section = ".iwram"]
#[instruction_set(arm::a32)]
pub unsafe fn a32_swp(in_val: u32, ptr: *mut u32) -> u32 {
  let out_val: u32;
  asm!(
    "swp {x}, {x}, [{address}]",
    address = in(reg) ptr,
    x = inout(reg) in_val => out_val,
    options(nostack, preserves_flags),
  );
  out_val
}

#[inline(never)]
#[link_section = ".iwram"]
#[instruction_set(arm::a32)]
pub unsafe fn a32_swpb(in_val: u8, ptr: *mut u8) -> u8 {
  let out_val: u8;
  asm!(
    "swpb {x}, {x}, [{address}]",
    address = in(reg) ptr,
    x = inout(reg) in_val => out_val,
    options(nostack, preserves_flags),
  );
  out_val
}

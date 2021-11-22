use core::{cell::UnsafeCell, mem::size_of};
use voladdress::{Safe, VolAddress, VolBlock};

use crate::macros::{
  const_new, u16_bool_field, u16_enum_field, u16_value_field,
};

mod bios;
pub use bios::*;

mod default_art;
pub use default_art::*;

#[derive(Debug, Clone, Copy)]
#[repr(u16)]
pub enum VideoMode {
  VideoMode0 = 0,
  VideoMode1 = 1,
  VideoMode2 = 2,
  VideoMode3 = 3,
  VideoMode4 = 4,
  VideoMode5 = 5,
}
pub use VideoMode::*;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct DisplayControl(u16);
impl DisplayControl {
  const_new!();
  u16_enum_field!(0 - 2: VideoMode, video_mode, with_video_mode);
  u16_bool_field!(4, display_frame1, with_display_frame1);
  u16_bool_field!(5, hblank_oam_free, with_hblank_oam_free);
  u16_bool_field!(6, obj_vram_1d, with_obj_vram_1d);
  u16_bool_field!(7, forced_blank, with_forced_blank);
  u16_bool_field!(8, display_bg0, with_display_bg0);
  u16_bool_field!(9, display_bg1, with_display_bg1);
  u16_bool_field!(10, display_bg2, with_display_bg2);
  u16_bool_field!(11, display_bg3, with_display_bg3);
  u16_bool_field!(12, display_obj, with_display_obj);
  u16_bool_field!(13, display_win0, with_display_win0);
  u16_bool_field!(14, display_win1, with_display_win1);
  u16_bool_field!(15, display_obj_win, with_display_obj_win);
}
pub const DISPCNT: VolAddress<DisplayControl, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_0000) };

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct DisplayStatus(u16);
impl DisplayStatus {
  const_new!();
  u16_bool_field!(0, is_vblank, with_is_vblank);
  u16_bool_field!(1, is_hblank, with_is_hblank);
  u16_bool_field!(2, is_vcounter_match, with_is_vcounter_match);
  u16_bool_field!(3, vblank_irq, with_vblank_irq);
  u16_bool_field!(4, hblank_irq, with_hblank_irq);
  u16_bool_field!(5, vcounter_irq, with_vcounter_irq);
  u16_value_field!(8 - 15, vcounter_setting, with_vcounter_setting);
}
pub const DISPSTAT: VolAddress<DisplayStatus, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_0004) };

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct KeysLowActive(u16);
pub const KEYINPUT: VolAddress<KeysLowActive, Safe, ()> =
  unsafe { VolAddress::new(0x0400_0130) };

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Keys(u16);
impl Keys {
  const_new!();
  u16_bool_field!(0, a, with_a);
  u16_bool_field!(1, b, with_b);
  u16_bool_field!(2, select, with_select);
  u16_bool_field!(3, start, with_start);
  u16_bool_field!(4, right, with_right);
  u16_bool_field!(5, left, with_left);
  u16_bool_field!(6, up, with_up);
  u16_bool_field!(7, down, with_down);
  u16_bool_field!(8, r, with_r);
  u16_bool_field!(9, l, with_l);
}
impl From<KeysLowActive> for Keys {
  #[inline]
  #[must_use]
  fn from(low: KeysLowActive) -> Self {
    Self(low.0 ^ 0b11_1111_1111)
  }
}
impl From<Keys> for u16 {
  #[inline]
  #[must_use]
  fn from(k: Keys) -> Self {
    k.0
  }
}
#[inline]
#[must_use]
pub fn get_keys() -> Keys {
  KEYINPUT.read().into()
}

pub const BACKDROP: VolAddress<Color, Safe, Safe> =
  unsafe { VolAddress::new(0x0500_0000) };

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Color(u16);
impl Color {
  u16_value_field!(0 - 4, red, with_red);
  u16_value_field!(5 - 9, green, with_green);
  u16_value_field!(10 - 14, blue, with_blue);
  #[inline]
  #[must_use]
  pub const fn from_rgb(r: u16, g: u16, b: u16) -> Self {
    Self(0).with_red(r).with_green(g).with_blue(b)
  }
  #[inline]
  #[must_use]
  pub const fn to_bits(self) -> u16 {
    self.0
  }
  #[inline]
  #[must_use]
  pub const fn from_bits(u: u16) -> Self {
    Self(u)
  }
  //
  pub const BLACK: Self = Self::from_rgb(0, 0, 0);
  pub const RED: Self = Self::from_rgb(31, 0, 0);
  pub const GREEN: Self = Self::from_rgb(0, 31, 0);
  pub const YELLOW: Self = Self::from_rgb(31, 31, 0);
  pub const BLUE: Self = Self::from_rgb(0, 0, 31);
  pub const MAGENTA: Self = Self::from_rgb(31, 0, 31);
  pub const CYAN: Self = Self::from_rgb(0, 31, 31);
  pub const WHITE: Self = Self::from_rgb(31, 31, 31);
  //
  pub const DIM_BLACK: Self = Self::from_rgb(10, 10, 10);
  pub const DIM_RED: Self = Self::from_rgb(21, 0, 0);
  pub const DIM_GREEN: Self = Self::from_rgb(0, 21, 0);
  pub const DIM_YELLOW: Self = Self::from_rgb(21, 21, 0);
  pub const DIM_BLUE: Self = Self::from_rgb(0, 0, 21);
  pub const DIM_MAGENTA: Self = Self::from_rgb(21, 0, 21);
  pub const DIM_CYAN: Self = Self::from_rgb(0, 21, 21);
  pub const DIM_WHITE: Self = Self::from_rgb(21, 21, 21);
}
impl From<u16> for Color {
  #[inline]
  #[must_use]
  fn from(u: u16) -> Self {
    Self::from_bits(u)
  }
}
impl From<Color> for u16 {
  #[inline]
  #[must_use]
  fn from(c: Color) -> Self {
    c.to_bits()
  }
}

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
}
pub type RustIrqFn = extern "C" fn(IrqBits);
extern "C" {
  static RUST_IRQ_HANDLER: GbaCell<Option<RustIrqFn>>;
}
#[inline(always)]
pub fn set_irq_handler(opt_fn: Option<RustIrqFn>) {
  unsafe { RUST_IRQ_HANDLER.write(opt_fn) }
}

pub const BG_PALETTE: VolBlock<Color, Safe, Safe, 256> =
  unsafe { VolBlock::new(0x0500_0000) };

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

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct TextScreenEntry(u16);
impl TextScreenEntry {
  const_new!();
  u16_value_field!(0 - 9, tile_id, with_tile_id);
  u16_bool_field!(10, h_flip, with_h_flip);
  u16_bool_field!(11, v_flip, with_v_flip);
  u16_value_field!(12 - 15, palbank, with_palbank);
}

pub type Tile4 = [u32; (4 * 8 * 8) / 32];
pub type Charblock4 = VolBlock<Tile4, Safe, Safe, 512>;

pub type Tile8 = [u32; (8 * 8 * 8) / 32];
pub type Charblock8 = VolBlock<Tile8, Safe, Safe, 256>;

#[inline]
#[must_use]
pub const fn charblock4<const N: usize>() -> Charblock4 {
  assert!(N < 6);
  unsafe { VolBlock::new(0x0600_0000 + N * size_of::<[Tile4; 512]>()) }
}

pub type TextScreenblock = VolBlock<TextScreenEntry, Safe, Safe, { 32 * 32 }>;

/// Gets a screenblock
///
/// Note: There's 8 screenblocks to a charblock.
#[inline]
#[must_use]
pub const fn text_screenblock<const N: usize>() -> TextScreenblock {
  assert!(N < 32);
  unsafe {
    VolBlock::new(0x0600_0000 + N * size_of::<[TextScreenEntry; 32 * 32]>())
  }
}

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

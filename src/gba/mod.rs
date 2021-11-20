use core::cell::UnsafeCell;

use crate::macros::{
  const_new, u16_bool_field, u16_enum_field, u16_value_field,
};

mod bios;
pub use bios::*;

#[repr(u16)]
pub enum VideoMode {
  VideoMode0 = 0,
  VideoMode1 = 1,
  VideoMode2 = 2,
  VideoMode3 = 3,
  VideoMode4 = 4,
  VideoMode5 = 5,
}
use voladdress::{Safe, VolAddress};
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
  pub fn read(&self) -> T {
    unsafe { self.0.get().read_volatile() }
  }
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

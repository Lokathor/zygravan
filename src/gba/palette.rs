use super::*;

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

/// The palette ram.
///
/// This type is a ZST that's just a nice way to collect all the palette ram
/// constructor methods into a single namespace.
#[derive(Debug, Clone, Copy)]
pub struct PalRam;
impl PalRam {
  #[inline]
  #[must_use]
  pub const fn backdrop() -> VolAddress<Color, Safe, Safe> {
    unsafe { VolAddress::new(0x0500_0000) }
  }
  #[inline]
  #[must_use]
  pub const fn bg() -> VolBlock<Color, Safe, Safe, 256> {
    unsafe { VolBlock::new(0x0500_0000) }
  }
  #[inline]
  #[must_use]
  pub const fn obj() -> VolBlock<Color, Safe, Safe, 256> {
    unsafe { VolBlock::new(0x0500_0200) }
  }
  #[inline]
  #[must_use]
  pub const fn try_bg_palbank(
    bank: usize,
  ) -> Option<VolBlock<Color, Safe, Safe, 16>> {
    if bank < 16 {
      Some(unsafe {
        VolBlock::new(0x0500_0000 + size_of::<[Color; 16]>() * bank)
      })
    } else {
      None
    }
  }
  #[inline]
  #[must_use]
  pub const fn try_obj_palbank(
    bank: usize,
  ) -> Option<VolBlock<Color, Safe, Safe, 16>> {
    if bank < 16 {
      Some(unsafe {
        VolBlock::new(0x0500_0200 + size_of::<[Color; 16]>() * bank)
      })
    } else {
      None
    }
  }
  #[inline]
  #[must_use]
  pub const fn bg_palbank(bank: usize) -> VolBlock<Color, Safe, Safe, 16> {
    match Self::try_bg_palbank(bank) {
      Some(block) => block,
      None => panic!("palbank index must be less than 16"),
    }
  }
  #[inline]
  #[must_use]
  pub const fn obj_palbank(bank: usize) -> VolBlock<Color, Safe, Safe, 16> {
    match Self::try_obj_palbank(bank) {
      Some(block) => block,
      None => panic!("palbank index must be less than 16"),
    }
  }
}

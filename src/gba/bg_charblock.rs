use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct BgCharblock(usize);
impl BgCharblock {
  pub const _0: Self = Self(0);
  pub const _1: Self = Self(1);
  pub const _2: Self = Self(2);
  pub const _3: Self = Self(3);

  /// Makes a new background charblock.
  ///
  /// Note that if you don't need to dynamically select a charblock, you can
  /// just use one of the associated constants, `_0` through `_3`.
  ///
  /// ## Failure
  /// * If your input is 4 or more.
  #[inline]
  #[must_use]
  pub const fn try_new(n: usize) -> Option<Self> {
    if n < 4 {
      Some(Self(n))
    } else {
      None
    }
  }

  #[inline]
  #[must_use]
  pub const fn tiles4(self) -> VolRegion<Tile4, Safe, Safe> {
    let len = match self.0 {
      0 | 1 | 2 => 1024,
      3 => 512,
      _ => panic!("illegal charblock value"),
    };
    let addr = unsafe { VolAddress::new(0x0600_0000 + ((16 * 1024) * self.0)) };
    unsafe { VolRegion::from_raw_parts(addr, len) }
  }

  #[inline]
  #[must_use]
  pub const fn tiles8(self) -> VolRegion<Tile8, Safe, Safe> {
    let len = match self.0 {
      0 | 1 => 1024,
      2 => 512,
      3 => 256,
      _ => panic!("illegal charblock value"),
    };
    let addr = unsafe { VolAddress::new(0x0600_0000 + ((16 * 1024) * self.0)) };
    unsafe { VolRegion::from_raw_parts(addr, len) }
  }
}

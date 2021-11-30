use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct TextScreenEntry(u16);
impl TextScreenEntry {
  const_new!();
  u16_value_field!(0 - 9, tile_id, with_tile_id);
  u16_bool_field!(10, h_flip, with_h_flip);
  u16_bool_field!(11, v_flip, with_v_flip);
  u16_value_field!(12 - 15, palbank, with_palbank);
  #[inline]
  #[must_use]
  pub const fn from_id_bank(id: u16, palbank: u16) -> Self {
    Self((id & 0x1_FF) | palbank << 12)
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct TextScreenblock(VolBlock<TextScreenEntry, Safe, Safe, { 32 * 32 }>);
impl TextScreenblock {
  pub const _0: Self = Self::new(0);
  pub const _1: Self = Self::new(1);
  pub const _2: Self = Self::new(2);
  pub const _3: Self = Self::new(3);
  pub const _4: Self = Self::new(4);
  pub const _5: Self = Self::new(5);
  pub const _6: Self = Self::new(6);
  pub const _7: Self = Self::new(7);
  pub const _8: Self = Self::new(8);
  pub const _9: Self = Self::new(9);
  pub const _10: Self = Self::new(10);
  pub const _11: Self = Self::new(11);
  pub const _12: Self = Self::new(12);
  pub const _13: Self = Self::new(13);
  pub const _14: Self = Self::new(14);
  pub const _15: Self = Self::new(15);
  pub const _16: Self = Self::new(16);
  pub const _17: Self = Self::new(17);
  pub const _18: Self = Self::new(18);
  pub const _19: Self = Self::new(19);
  pub const _20: Self = Self::new(20);
  pub const _21: Self = Self::new(21);
  pub const _22: Self = Self::new(22);
  pub const _23: Self = Self::new(23);
  pub const _24: Self = Self::new(24);
  pub const _25: Self = Self::new(25);
  pub const _26: Self = Self::new(26);
  pub const _27: Self = Self::new(27);
  pub const _28: Self = Self::new(28);
  pub const _29: Self = Self::new(29);
  pub const _30: Self = Self::new(30);
  pub const _31: Self = Self::new(31);

  /// Makes a new text screenblock.
  ///
  /// ## Failure
  /// * If your input is 32 or more.
  #[inline]
  #[must_use]
  pub const fn try_new(n: usize) -> Option<Self> {
    if n < 32 {
      Some(Self(unsafe {
        VolBlock::new(0x0600_0000 + n * size_of::<[TextScreenEntry; 32 * 32]>())
      }))
    } else {
      None
    }
  }
  #[inline]
  #[must_use]
  pub const fn new(n: usize) -> Self {
    match Self::try_new(n) {
      Some(tsb) => tsb,
      None => panic!("text screenblock index must be less than 32"),
    }
  }

  #[inline]
  pub fn write_all(self, tse: TextScreenEntry) {
    self.0.iter().for_each(|va| va.write(tse))
  }

  #[inline]
  #[must_use]
  pub const fn as_volblock(
    &self,
  ) -> &VolBlock<TextScreenEntry, Safe, Safe, { 32 * 32 }> {
    &self.0
  }
}

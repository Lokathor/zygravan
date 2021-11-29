//! Fixed point math stuff.

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Fx<T, const BITS: usize>(T);

macro_rules! impl_common_fixed_point_stuff {
  ($($t:ty),+) => {
    $(
      impl<const BITS: usize> core::ops::Add for Fx<$t, BITS> {
        type Output = Self;
        #[inline]
        #[must_use]
        fn add(self, rhs: Self) -> Self {
          Self(self.0.add(rhs.0))
        }
      }
      impl<const BITS: usize> core::ops::AddAssign for Fx<$t, BITS> {
        #[inline]
        #[must_use]
        fn add_assign(&mut self, rhs: Self) {
          self.0.add_assign(rhs.0)
        }
      }
      impl<const BITS: usize> core::ops::Sub for Fx<$t, BITS> {
        type Output = Self;
        #[inline]
        #[must_use]
        fn sub(self, rhs: Self) -> Self {
          Self(self.0.sub(rhs.0))
        }
      }
      impl<const BITS: usize> core::ops::SubAssign for Fx<$t, BITS> {
        #[inline]
        #[must_use]
        fn sub_assign(&mut self, rhs: Self) {
          self.0.sub_assign(rhs.0)
        }
      }
      impl<const BITS: usize> core::ops::Mul for Fx<$t, BITS> {
        type Output = Self;
        #[inline]
        #[must_use]
        fn mul(self, rhs: Self) -> Self {
          Self(self.0.mul(rhs.0) >> BITS)
        }
      }
      impl<const BITS: usize> core::ops::MulAssign for Fx<$t, BITS> {
        #[inline]
        #[must_use]
        fn mul_assign(&mut self, rhs: Self) {
          use core::ops::Mul;
          *self = self.mul(rhs);
        }
      }
      impl<const BITS: usize> Fx<$t, BITS> {
        #[inline] #[must_use]
        pub const fn new(n: $t) -> Self {
          Self(n << BITS)
        }
        #[inline] #[must_use]
        pub const fn as_fx_i8(self) -> Fx<i8, BITS> {
          Fx(self.0 as i8)
        }
        #[inline] #[must_use]
        pub const fn as_fx_i16(self) -> Fx<i16, BITS> {
          Fx(self.0 as i16)
        }
        #[inline] #[must_use]
        pub const fn as_fx_i32(self) -> Fx<i32, BITS> {
          Fx(self.0 as i32)
        }
        #[inline] #[must_use]
        pub const fn as_fx_u8(self) -> Fx<u8, BITS> {
          Fx(self.0 as u8)
        }
        #[inline] #[must_use]
        pub const fn as_fx_u16(self) -> Fx<u16, BITS> {
          Fx(self.0 as u16)
        }
        #[inline] #[must_use]
        pub const fn as_fx_u32(self) -> Fx<u32, BITS> {
          Fx(self.0 as u32)
        }
        #[inline] #[must_use]
        pub const fn wrapping_add(self, rhs: Self) -> Self {
          Fx(self.0.wrapping_add(rhs.0))
        }
        #[inline] #[must_use]
        pub const fn wrapping_sub(self, rhs: Self) -> Self {
          Fx(self.0.wrapping_sub(rhs.0))
        }
        #[inline] #[must_use]
        pub const fn saturating_add(self, rhs: Self) -> Self {
          Fx(self.0.saturating_add(rhs.0))
        }
        #[inline] #[must_use]
        pub const fn saturating_sub(self, rhs: Self) -> Self {
          Fx(self.0.saturating_sub(rhs.0))
        }
        #[inline] #[must_use]
        pub const fn checked_add(self, rhs: Self) -> Option<Self> {
          match self.0.checked_add(rhs.0) {
            Some(n) => Some(Fx(n)),
            None => None,
          }
        }
        #[inline] #[must_use]
        pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
          match self.0.checked_sub(rhs.0) {
            Some(n) => Some(Fx(n)),
            None => None,
          }
        }
        #[inline] #[must_use]
        pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
          let (n, b) = self.0.overflowing_add(rhs.0);
          (Fx(n), b)
        }
        #[inline] #[must_use]
        pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
          let (n, b) = self.0.overflowing_sub(rhs.0);
          (Fx(n), b)
        }
        //
      }
    )+
  }
}
impl_common_fixed_point_stuff! {
  i8, u8, i16, u16, i32, u32
}

macro_rules! impl_signed_fixed_point_stuff {
  ($($t:ty),+) => {
    $(
      impl<const BITS: usize> core::ops::Neg for Fx<$t, BITS> {
        type Output = Self;
        #[inline]
        #[must_use]
        fn neg(self) -> Self {
          Self(self.0.neg())
        }
      }
      impl<const BITS: usize> Fx<$t, BITS> {
        #[inline] #[must_use]
        pub const fn abs(self) -> Self {
          Self(self.0.abs())
        }
        #[inline] #[must_use]
        pub const fn wrapping_abs(self) -> Self {
          Self(self.0.wrapping_abs())
        }
        #[inline] #[must_use]
        pub const fn signum(self) -> Self {
          Self::new(self.0.signum())
        }
        //
      }
    )+
  };
}
impl_signed_fixed_point_stuff! {
  i8, i16, i32
}

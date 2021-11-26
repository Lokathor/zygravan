use super::*;

/// A dynamically sized span of volatile memory.
///
/// If you think of [VolBlock] as being similar to an array, this type is more
/// similar to a slice.
///
/// The primary utility of this type is just that it bundles a pointer and
/// length together, which allows you to have safe dynamic bounds checking. It
/// does **not** have a lifetime or participate in borrow checking, and it does
/// **not** enforce exclusive access.
///
/// A `VolRegion` assumes that elements of the region are directly one after the
/// other, like how `VolBlock` works. If you need dynamic bounds checking on a
/// spaced out series of values that would be some other type, which doesn't
/// currently exist in the library. Open a PR maybe.
#[repr(C)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VolRegion<T, R, W> {
  addr: VolAddress<T, R, W>,
  len: usize,
}
impl<T, R, W> Clone for VolRegion<T, R, W> {
  #[inline]
  #[must_use]
  fn clone(&self) -> Self {
    *self
  }
}
impl<T, R, W> Copy for VolRegion<T, R, W> {}
impl<T, R, W> core::fmt::Debug for VolRegion<T, R, W> {
  #[cold]
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "VolRegion<{elem_ty}, r{readability}, w{writeability}>({address:#X}, {len})",
      elem_ty = core::any::type_name::<T>(),
      readability=core::any::type_name::<R>(),
      writeability=core::any::type_name::<W>(),
      address=self.addr.as_usize(),
      len=self.len,
    )
  }
}
impl<T, R, W, const C: usize> From<VolBlock<T, R, W, C>>
  for VolRegion<T, R, W>
{
  #[inline]
  #[must_use]
  fn from(block: VolBlock<T, R, W, C>) -> Self {
    // Note: temporary hack to go around privacy issues. In the future this
    // should just read `block.base`
    Self { addr: unsafe { core::mem::transmute(block) }, len: C }
  }
}

impl<T, R, W> VolRegion<T, R, W> {
  #[inline]
  #[must_use]
  pub const unsafe fn from_raw_parts(
    addr: VolAddress<T, R, W>, len: usize,
  ) -> Self {
    Self { addr, len }
  }

  #[inline]
  #[must_use]
  pub const fn addr(self) -> VolAddress<T, R, W> {
    self.addr
  }

  #[inline]
  #[must_use]
  pub const fn len(self) -> usize {
    self.len
  }

  #[inline]
  #[must_use]
  #[track_caller]
  pub const fn index(self, i: usize) -> VolAddress<T, R, W> {
    if i < self.len {
      unsafe { self.addr.add(i) }
    } else {
      // Note(Lokathor): We force a const panic by indexing out of bounds.
      #[allow(unconditional_panic)]
      unsafe {
        VolAddress::new([usize::MAX][1])
      }
    }
  }

  #[inline]
  #[must_use]
  pub const fn get(self, i: usize) -> Option<VolAddress<T, R, W>> {
    if i < self.len {
      Some(unsafe { self.addr.add(i) })
    } else {
      None
    }
  }

  #[inline]
  #[must_use]
  #[track_caller]
  pub fn sub_slice<RB: core::ops::RangeBounds<usize>>(self, r: RB) -> Self {
    // TODO: some day make this a const fn, once start_bound and end_bound are
    // made into const fn, but that requires const trait impls.
    use core::ops::Bound;
    let start_inclusive: usize = match r.start_bound() {
      Bound::Included(i) => *i,
      Bound::Excluded(x) => x + 1,
      Bound::Unbounded => 0,
    };
    assert!(start_inclusive < self.len);
    let end_exclusive: usize = match r.end_bound() {
      Bound::Included(i) => i + 1,
      Bound::Excluded(x) => *x,
      Bound::Unbounded => self.len,
    };
    assert!(end_exclusive <= self.len);
    let len = end_exclusive.saturating_sub(start_inclusive);
    Self { addr: unsafe { self.addr.add(start_inclusive) }, len }
  }

  #[inline]
  #[must_use]
  pub const fn iter(self) -> VolBlockIter<T, R, W> {
    // Note: temporary hack to go around privacy issues. In the future this
    // should just construct the iterator normally.
    unsafe { core::mem::transmute(self) }
  }

  #[inline]
  #[must_use]
  #[track_caller]
  pub fn iter_range<RB: core::ops::RangeBounds<usize>>(
    self, r: RB,
  ) -> VolBlockIter<T, R, W> {
    self.sub_slice(r).iter()
  }
}

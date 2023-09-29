#[derive(Clone, Copy, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct Player(u8);

impl Player {
  pub const TOTAL: usize = 0x0C;

  #[inline]
  pub const fn from_u8(value: u8) -> Option<Self> {
    if value < Self::TOTAL as u8 {
      // SAFETY: We just checked the validity of `value`.
      Some(unsafe { Self::from_u8_unchecked(value) })
    } else {
      None
    }
  }

  #[inline]
  pub const unsafe fn from_u8_unchecked(value: u8) -> Self {
    Self(value)
  }

  #[inline]
  pub const fn as_u8(self) -> u8 {
    self.0
  }

  #[inline]
  pub const fn as_usize(self) -> usize {
    self.0 as usize
  }
}

use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;
use core::ops::DerefMut;

use crate::consts::MD5_DIGEST_SIZE;
use crate::utils::Hex;

pub type DigestMd5 = Digest<{ MD5_DIGEST_SIZE }>;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Digest<const S: usize>([u8; S]);

impl<const S: usize> Digest<S> {
  /// The size of a `Digest<S>`.
  pub const SIZE: usize = S;

  /// Create a new empty `Digest<S>`.
  #[inline]
  pub const fn empty() -> Self {
    Self([0; S])
  }

  /// Returns a `Hex` formatter for the digest.
  #[inline]
  pub const fn as_hex(&self) -> Hex<'_, S> {
    // TODO: S * 2
    Hex::new(&self.0)
  }

  /// Returns the digest as a slice of bytes.
  #[inline]
  pub const fn as_slice(&self) -> &[u8] {
    self.0.as_slice()
  }
}

impl<const S: usize> Debug for Digest<S> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Debug::fmt(&self.as_hex(), f)
  }
}

impl<const S: usize> Display for Digest<S> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Display::fmt(&self.as_hex(), f)
  }
}

impl<const S: usize> Deref for Digest<S> {
  type Target = [u8; S];

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<const S: usize> DerefMut for Digest<S> {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl<const S: usize> AsRef<[u8]> for Digest<S> {
  #[inline]
  fn as_ref(&self) -> &[u8] {
    &self.0
  }
}

impl<const S: usize> From<[u8; S]> for Digest<S> {
  #[inline]
  fn from(other: [u8; S]) -> Self {
    Self(other)
  }
}

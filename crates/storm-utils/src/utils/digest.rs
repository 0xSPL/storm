use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;
use core::ops::DerefMut;

use crate::consts::MD5_DIGEST_SIZE;

pub type DigestMd5 = Digest<{ MD5_DIGEST_SIZE }>;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Digest<const S: usize>([u8; S]);

impl<const S: usize> Digest<S> {
  pub const SIZE: usize = S;

  /// Create a new empty digest.
  #[inline]
  pub const fn empty() -> Self {
    Self([0; S])
  }

  /// Returns the digest as a slice of bytes.
  #[inline]
  pub const fn as_slice(&self) -> &[u8] {
    self.0.as_slice()
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

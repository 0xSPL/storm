use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;
use core::ops::DerefMut;
use md5::Digest as _;
use md5::Md5;

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

impl DigestMd5 {
  /// Create a new `DigestMd5` from the given `data`.
  #[inline]
  pub fn new(input: &[u8]) -> Self {
    Self::build(|hasher| hasher.update(input))
  }

  pub fn build(f: impl Fn(&mut Hasher<Md5>)) -> Self {
    let mut hasher: Md5 = Md5::new();
    let mut output: Self = Self::empty();

    f(&mut Hasher::new(&mut hasher));

    hasher.finalize_into((&mut output[..]).into());
    output
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

// =============================================================================
// Hasher
// =============================================================================

pub struct Hasher<'a, D> {
  hasher: &'a mut D,
}

impl<'a, D: md5::Digest> Hasher<'a, D> {
  /// Create a new `Hasher`.
  #[inline]
  pub fn new(hasher: &'a mut D) -> Self {
    Self { hasher }
  }

  /// Update the hasher state with the given `data`.
  #[inline]
  pub fn update(&mut self, data: impl AsRef<[u8]>) {
    self.hasher.update(data);
  }
}

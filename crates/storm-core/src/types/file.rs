use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result;
use core::ops::Deref;

// =============================================================================
// File
// =============================================================================

/// A file extracted from an `Archive`.
#[derive(Clone, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct File {
  data: Vec<u8>,
}

impl File {
  /// Create an new, empty `File`.
  #[inline]
  pub const fn empty() -> Self {
    Self::new(Vec::new())
  }

  /// Create a new `File` containing the given `data`.
  #[inline]
  pub const fn new(data: Vec<u8>) -> Self {
    Self { data }
  }

  /// Returns the size of the file (in bytes).
  #[inline]
  pub fn size(&self) -> usize {
    self.data.len()
  }

  /// Consumes the `File`, returning the contents as a `Vec<u8>`.
  #[inline]
  pub fn into_vec(self) -> Vec<u8> {
    self.data
  }
}

impl Deref for File {
  type Target = [u8];

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.data
  }
}

impl Debug for File {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("File")
      .field("size", &self.size())
      .finish_non_exhaustive()
  }
}

use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;

/// Error encountered during compression/decompression.
#[derive(Debug)]
pub struct Error {
  kind: ErrorKind,
}

impl Error {
  #[inline]
  pub(crate) const fn new(kind: ErrorKind) -> Self {
    Self { kind }
  }

  /// Returns the corresponding [`ErrorKind`] for this error.
  #[inline]
  pub const fn kind(&self) -> ErrorKind {
    self.kind
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self.kind {
      ErrorKind::Compression => write!(f, "compression failed"),
      ErrorKind::Decompression => write!(f, "decompression failed"),
    }
  }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

/// The general category of [`Error`].
#[derive(Clone, Copy, Debug)]
pub enum ErrorKind {
  /// Fatal error during compression.
  Compression,
  /// Fatal error during decompression.
  Decompression,
}

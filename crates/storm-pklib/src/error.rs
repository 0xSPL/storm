use core::ffi::c_uint;
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
      ErrorKind::InvalidDict => write!(f, "invalid dictionary size"),
      ErrorKind::InvalidMode => write!(f, "invalid compression mode"),
      ErrorKind::InvalidData => write!(f, "invalid input data"),
      ErrorKind::Fatal => write!(f, "fatal error encountered"),
      ErrorKind::Unknown(status) => write!(f, "unknown error: {status}"),
    }
  }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

/// The general category of [`Error`].
#[derive(Clone, Copy, Debug)]
pub enum ErrorKind {
  /// Invalid dictionary size.
  InvalidDict,
  /// Invalid compression mode.
  InvalidMode,
  /// Invalid input data.
  InvalidData,
  /// Fatal error during compression.
  Fatal,
  /// Unknown error encountered.
  Unknown(c_uint),
}

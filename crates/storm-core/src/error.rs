use std::error::Error as StdError;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use crate::utils::CompressionFormat;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub struct Error {
  kind: ErrorKind,
  from: ErrorSource,
}

impl Error {
  #[doc(hidden)]
  #[inline]
  pub const fn new(kind: ErrorKind) -> Self {
    Self {
      kind,
      from: ErrorSource::Ignore,
    }
  }

  #[doc(hidden)]
  #[inline]
  pub fn new_std(kind: ErrorKind, source: impl StdError + 'static) -> Self {
    Self {
      kind,
      from: ErrorSource::Source(Box::new(source)),
    }
  }

  #[doc(hidden)]
  #[inline]
  pub fn message(message: impl Display) -> Self {
    Self {
      kind: ErrorKind::Other,
      from: ErrorSource::String(message.to_string()),
    }
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
      ErrorKind::InvalidIO => write!(f, "i/o error: {}", self.from),
      ErrorKind::InvalidUtf8 => write!(f, "invalid utf8: {}", self.from),
      ErrorKind::InvalidMagic => write!(f, "invalid magic signature"),
      ErrorKind::InvalidLen(name) => write!(f, "invalid md5 for {name}"),
      ErrorKind::InvalidMd5(name) => write!(f, "invalid len for {name}"),
      ErrorKind::FileInvalidSize => write!(f, "file invalid: bad size"),
      ErrorKind::FileInvalidType => write!(f, "file invalid: bad type"),
      ErrorKind::FileCorruptData => write!(f, "file corrupted/unreadable"),
      ErrorKind::FileDataMissing => write!(f, "file not found"),
      ErrorKind::DecompressionInvalid(mode) => {
        write!(f, "invalid decompression algorithm: {mode:#04X}")
      }
      ErrorKind::DecompressionFeature(format) => {
        write!(
          f,
          "enable `{}` feature to use {}",
          format.feature(),
          format.name()
        )
      }
      ErrorKind::DecompressionNoBytes => {
        write!(f, "attempted decompression on empty buffer")
      }
      ErrorKind::DecompressionFailure => {
        write!(f, "decompression failed: {}", self.from)
      }
      ErrorKind::DecompressionStatus(status) => {
        write!(f, "decompression failed: {status}")
      }
      ErrorKind::Other => {
        write!(f, "{}", self.from)
      }
    }
  }
}

impl StdError for Error {
  #[inline]
  fn source(&self) -> Option<&(dyn StdError + 'static)> {
    match self.from {
      ErrorSource::Ignore => None,
      ErrorSource::String(_) => None,
      ErrorSource::Source(ref source) => Some(&**source),
    }
  }
}

impl From<std::io::Error> for Error {
  #[inline]
  fn from(other: std::io::Error) -> Self {
    Self::new_std(ErrorKind::InvalidIO, other)
  }
}

impl From<std::string::FromUtf8Error> for Error {
  #[inline]
  fn from(other: std::string::FromUtf8Error) -> Self {
    Self::new_std(ErrorKind::InvalidUtf8, other)
  }
}

impl From<std::str::Utf8Error> for Error {
  #[inline]
  fn from(other: std::str::Utf8Error) -> Self {
    Self::new_std(ErrorKind::InvalidUtf8, other)
  }
}

#[derive(Clone, Copy, Debug)]
pub enum ErrorKind {
  // ===========================================================================
  // Parse Errors
  // ===========================================================================
  InvalidIO,
  InvalidUtf8,
  InvalidMagic,
  // ===========================================================================
  // Parse Errors (v4)
  // ===========================================================================
  InvalidLen(&'static str),
  InvalidMd5(&'static str),
  // ===========================================================================
  // File Errors
  // ===========================================================================
  FileInvalidSize,
  FileInvalidType,
  FileCorruptData,
  FileDataMissing,
  // ===========================================================================
  // Decompression Errors
  // ===========================================================================
  DecompressionInvalid(u8),
  DecompressionFeature(CompressionFormat),
  DecompressionNoBytes,
  DecompressionFailure,
  DecompressionStatus(&'static str),
  // ===========================================================================
  // Misc.
  // ===========================================================================
  Other,
}

#[derive(Debug)]
enum ErrorSource {
  Ignore,
  String(String),
  Source(Box<dyn StdError + 'static>),
}

impl Display for ErrorSource {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::Ignore => Ok(()),
      Self::String(inner) => Display::fmt(inner, f),
      Self::Source(inner) => Display::fmt(inner, f),
    }
  }
}

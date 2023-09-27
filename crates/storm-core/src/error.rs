use std::error::Error as StdError;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub struct Error {
  kind: ErrorKind,
  from: ErrorSource,
}

impl Error {
  #[inline]
  pub(crate) const fn new(kind: ErrorKind) -> Self {
    Self {
      kind,
      from: ErrorSource::Ignore,
    }
  }

  #[inline]
  pub(crate) fn new_std<T>(kind: ErrorKind, source: T) -> Self
  where
    T: StdError + 'static,
  {
    Self {
      kind,
      from: ErrorSource::Source(Box::new(source)),
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
      ErrorKind::DecompressionFeature(feature, name) => {
        write!(f, "enable `{feature}` feature to use {name}")
      }
      ErrorKind::DecompressionNoBytes => {
        write!(f, "attempted decompression on empty buffer")
      }
      ErrorKind::DecompressionFailure => {
        write!(f, "decompression failed: {}", self.from)
      }
    }
  }
}

impl StdError for Error {
  #[inline]
  fn source(&self) -> Option<&(dyn StdError + 'static)> {
    match self.from {
      ErrorSource::Ignore => None,
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

#[derive(Clone, Copy, Debug)]
pub enum ErrorKind {
  // ===========================================================================
  // Parse Errors
  // ===========================================================================
  InvalidIO,
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
  DecompressionFeature(&'static str, &'static str),
  DecompressionNoBytes,
  DecompressionFailure,
}

#[derive(Debug)]
enum ErrorSource {
  Ignore,
  Source(Box<dyn StdError + 'static>),
}

impl Display for ErrorSource {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::Ignore => Ok(()),
      Self::Source(inner) => Display::fmt(inner, f),
    }
  }
}

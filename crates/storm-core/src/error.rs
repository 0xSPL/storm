#[cfg(feature = "std")]
use std::error::Error as StdError;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub struct Error {
  kind: ErrorKind,
  from: Option<ErrorSource>,
}

impl Error {
  #[inline]
  pub(crate) const fn new(kind: ErrorKind) -> Self {
    Self { kind, from: None }
  }

  #[cfg(feature = "std")]
  #[inline]
  pub(crate) fn new_std<T>(kind: ErrorKind, source: T) -> Self
  where
    T: StdError + 'static,
  {
    Self {
      kind,
      from: Some(ErrorSource::Source(Box::new(source))),
    }
  }

  #[cfg(not(feature = "std"))]
  #[inline]
  pub(crate) fn new_std<T>(kind: ErrorKind, _source: T) -> Self {
    Self::new(kind)
  }

  /// Returns the corresponding [`ErrorKind`] for this error.
  #[inline]
  pub const fn kind(&self) -> ErrorKind {
    self.kind
  }
}

#[derive(Clone, Copy, Debug)]
pub enum ErrorKind {
  InvalidIO,
  InvalidMagic,
}

#[derive(Debug)]
enum ErrorSource {
  #[cfg(feature = "std")]
  Source(Box<dyn StdError + 'static>),
}

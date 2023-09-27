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

  /// Returns the corresponding [`ErrorKind`] for this error.
  #[inline]
  pub const fn kind(&self) -> ErrorKind {
    self.kind
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
  InvalidIO,
  InvalidMagic,
}

#[derive(Debug)]
enum ErrorSource {
  Source(Box<dyn StdError + 'static>),
}

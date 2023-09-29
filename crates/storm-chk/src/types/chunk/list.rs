use core::ops::Deref;
use storm_core::error::Error;
use storm_core::error::Result;
use storm_core::extract::FilePtr;
use storm_core::types::File;

use crate::parse::Parser;
use crate::types::Chunk;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ChunkList {
  inner: Box<[Chunk]>,
}

impl ChunkList {
  #[inline]
  pub fn new(data: File) -> Result<Self> {
    Ok(Self {
      inner: Parser::new(data.to_vec()).read_all()?,
    })
  }

  #[inline]
  pub const fn len(&self) -> usize {
    self.inner.len()
  }

  #[inline]
  pub const fn is_empty(&self) -> bool {
    self.inner.is_empty()
  }
}

impl Deref for ChunkList {
  type Target = [Chunk];

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

impl TryFrom<FilePtr<'_>> for ChunkList {
  type Error = Error;

  #[inline]
  fn try_from(other: FilePtr<'_>) -> Result<Self, Self::Error> {
    other.read().and_then(Self::new)
  }
}

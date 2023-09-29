use storm_core::error::Error;
use storm_core::error::Result;
use storm_core::extract::FilePtr;
use storm_core::types::File;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ChunkFile {}

impl ChunkFile {
  pub fn new(data: File) -> Result<Self> {
    panic!("ChunkFile::new");
  }
}

impl TryFrom<FilePtr<'_>> for ChunkFile {
  type Error = Error;

  #[inline]
  fn try_from(other: FilePtr<'_>) -> Result<Self, Self::Error> {
    other.read().and_then(Self::new)
  }
}

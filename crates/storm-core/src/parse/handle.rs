use std::fs::File;
use std::fs::Metadata;
use std::io;
use std::io::BufReader;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::path::Path;
use std::path::PathBuf;

use crate::error::Error;
use crate::error::ErrorKind;
use crate::error::Result;
use crate::types::HeaderV1;

// =============================================================================
// File Handle
// =============================================================================

#[derive(Debug)]
pub struct Handle {
  file: BufReader<File>,
  path: PathBuf,
  size: u64,
}

impl Handle {
  // Use a buffer size of 8KB - for performance.
  const BUFFER: usize = 0x2000;

  // Don't try to parse files larger than 128 MB.
  const MAX: u64 = 0x08000000;

  // All MPQs MUST have a header (at minimum).
  const MIN: u64 = HeaderV1::SIZE as u64;

  /// Create a new handle from the file at the given `path`.
  pub fn new<P>(path: &P) -> Result<Self>
  where
    P: AsRef<Path> + ?Sized,
  {
    let path: &Path = path.as_ref();

    // Ensure this path points to a real file
    if !path.is_file() {
      return Err(Error::new(ErrorKind::FileInvalidType));
    }

    let file: File = File::open(path)?;
    let meta: Metadata = file.metadata()?;
    let size: u64 = meta.len();

    // Check the file size and don't read anything invalid
    if !(Self::MIN..=Self::MAX).contains(&size) {
      return Err(Error::new(ErrorKind::FileInvalidSize));
    }

    Ok(Self {
      file: Self::create_reader(Self::BUFFER, file),
      path: path.to_owned(),
      size,
    })
  }

  /// Returns a reference to the file.
  #[inline]
  pub fn file(&self) -> &File {
    self.file.get_ref()
  }

  /// Returns the path to the file.
  #[inline]
  pub fn path(&self) -> &Path {
    self.path.as_path()
  }

  /// Returns the size (in bytes) of the file.
  #[inline]
  pub const fn size(&self) -> u64 {
    self.size
  }

  /// Returns the byte capacity of the internal buffer.
  #[inline]
  pub fn capacity(&self) -> usize {
    self.file.capacity()
  }

  /// Create a clone of the file handle with the same buffer capacity.
  #[inline]
  pub fn duplicate_exact(&self) -> Result<Self> {
    self.duplicate(Self::BUFFER)
  }

  /// Create a clone of the file handle with the specified buffer `capacity`.
  #[inline]
  pub fn duplicate(&self, capacity: usize) -> Result<Self> {
    Ok(Self {
      file: self.clone_file(capacity)?,
      path: self.path.clone(),
      size: self.size,
    })
  }

  #[inline]
  fn clone_file(&self, capacity: usize) -> Result<BufReader<File>> {
    self
      .file()
      .try_clone()
      .map(|file| Self::create_reader(capacity, file))
      .map_err(Into::into)
  }

  #[inline]
  fn create_reader(capacity: usize, file: File) -> BufReader<File> {
    BufReader::with_capacity(capacity, file)
  }
}

impl Read for Handle {
  #[inline]
  fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
    self.file.read(buffer)
  }
}

impl Seek for Handle {
  #[inline]
  fn seek(&mut self, from: SeekFrom) -> io::Result<u64> {
    self.file.seek(from)
  }
}

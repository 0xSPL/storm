use std::io::Cursor;
use storm_utils::bitflags;
use storm_utils::traits::ReadExt;
use storm_utils::utils::DigestMd5;

use crate::error::Error;
use crate::error::Result;
use crate::extract::FilePtr;
use crate::types::File;
use crate::utils::convert_filetime;

only_serde! {
  use serde::ser::SerializeStruct;
  use serde::Serialize;
  use serde::Serializer;
}

// =============================================================================
// Attribute File
// =============================================================================

/// Optional attributes for files in the block table.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct AttrFile {
  /// Specifies the extended attributes format version.
  pub version: u32,
  /// The extended attributes present in the archive.
  pub bitflags: AttrFlags,
  /// CRC32s of the file data (uncompressed) for each block in the archive.
  pub crc: Box<[u32]>,
  /// Timestamps for each block in the archive.
  pub time: Box<[u64]>,
  /// MD5s of the file data (uncompressed) for each block in the archive.
  pub md5: Box<[DigestMd5]>,
}

impl AttrFile {
  pub fn new(file: File, entries: u32) -> Result<Self> {
    let mut reader: Cursor<Vec<u8>> = Cursor::new(file.into_vec());

    let version: u32 = reader.read_u32_le()?;
    let bitflags: AttrFlags = AttrFlags::from_value(reader.read_u32_le()?);

    Ok(Self {
      version,
      bitflags,
      crc: read_crc(&mut reader, entries, bitflags)?,
      time: read_time(&mut reader, entries, bitflags)?,
      md5: read_md5(&mut reader, entries, bitflags)?,
    })
  }
}

impl TryFrom<FilePtr<'_>> for AttrFile {
  type Error = Error;

  #[inline]
  fn try_from(other: FilePtr<'_>) -> Result<Self, Self::Error> {
    let size: u32 = other.archive.header.btable_entries;
    let data: File = other.read()?;

    Self::new(data, size)
  }
}

only_serde! {
  impl Serialize for AttrFile {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
      let mut state: S::SerializeStruct = serializer.serialize_struct("AttrFile", 5)?;
      state.serialize_field("version", &self.version)?;
      state.serialize_field("bitflags", &self.bitflags)?;
      state.serialize_field("crc", &self.crc)?;
      state.serialize_field("time", &self.time)?;
      state.serialize_field("md5", &self.md5)?;
      state.end()
    }
  }
}

// =============================================================================
// Attribute Flags
// =============================================================================

bitflags! {
  /// Attribute File Flags.
  #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
  pub struct AttrFlags: u32 {
    /// File has CRC32 attributes.
    const CRC = 0x00000001;
    /// File has timestamp attributes.
    const TIME = 0x00000002;
    /// File has md5 attributes.
    const MD5 = 0x00000004;
  }
}

only_serde! {
  impl Serialize for AttrFlags {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
      serializer.collect_str(self)
    }
  }
}

// =============================================================================
// Parsing
// =============================================================================

fn read_crc<R: ReadExt>(reader: &mut R, entries: u32, bitflags: AttrFlags) -> Result<Box<[u32]>> {
  if !bitflags.contains(AttrFlags::CRC) {
    return Ok(Box::new([]));
  }

  let mut data: Vec<u32> = Vec::new();

  // Allocate space for all entries
  data.reserve(entries as usize);

  for _ in 0..entries {
    data.push(reader.read_u32_le()?);
  }

  Ok(data.into_boxed_slice())
}

fn read_time<R: ReadExt>(reader: &mut R, entries: u32, bitflags: AttrFlags) -> Result<Box<[u64]>> {
  if !bitflags.contains(AttrFlags::TIME) {
    return Ok(Box::new([]));
  }

  let mut data: Vec<u64> = Vec::new();

  // Allocate space for all entries
  data.reserve(entries as usize);

  for _ in 0..entries {
    let lo: u32 = reader.read_u32_le()?;
    let hi: u32 = reader.read_u32_le()?;

    if lo == 0 && hi == 0 {
      data.push(0);
    } else {
      data.push(convert_filetime(lo, hi));
    }
  }

  Ok(data.into_boxed_slice())
}

fn read_md5<R: ReadExt>(
  reader: &mut R,
  entries: u32,
  bitflags: AttrFlags,
) -> Result<Box<[DigestMd5]>> {
  if !bitflags.contains(AttrFlags::MD5) {
    return Ok(Box::new([]));
  }

  let mut data: Vec<DigestMd5> = Vec::new();

  // Allocate space for all entries
  data.reserve(entries as usize);

  for _ in 0..entries {
    data.push(reader.read_array_u8()?.into());
  }

  Ok(data.into_boxed_slice())
}

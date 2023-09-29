use storm_core::error::Result;
use storm_utils::traits::ReadExt;

use crate::parse::BoxedSize;
use crate::parse::ChunkType;
use crate::parse::ParseChunk;
use crate::types::Item;

// =============================================================================
// Locations
// =============================================================================

/// This section contains all the locations that the map uses.
///
/// Required for all versions. Not required for Melee.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Mrgn {
  pub locations: Box<[MrgnData]>,
}

impl From<Mrgn> for Item {
  #[inline]
  fn from(other: Mrgn) -> Self {
    Self::Mrgn(other)
  }
}

impl ParseChunk for Mrgn {
  const TYPE: ChunkType = ChunkType::Boxed(BoxedSize::Int(0x14));

  fn from_reader<R: ReadExt>(reader: &mut R, size: u32) -> Result<Self> {
    Ok(Self {
      locations: Self::read_boxed(reader, size, MrgnData::from_reader)?,
    })
  }
}

// =============================================================================
// Mrgn Data
// =============================================================================

// TODO: bitflags
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct MrgnData {
  /// Left (X1) coordinate of location.
  pub x1: u32,
  /// Top (Y1) coordinate of location.
  pub y1: u32,
  /// Right (X2) coordinate of location.
  pub x2: u32,
  /// Bottom (Y2) coordinate of location.
  pub y2: u32,
  /// String number of the name of this location.
  pub index: u16,
  /// Location elevation flags.
  pub flags: u16,
}

impl MrgnData {
  pub fn from_reader<R: ReadExt>(reader: &mut R) -> Result<Self> {
    Ok(Self {
      x1: reader.read_u32_le()?,
      y1: reader.read_u32_le()?,
      x2: reader.read_u32_le()?,
      y2: reader.read_u32_le()?,
      index: reader.read_u16_le()?,
      flags: reader.read_u16_le()?,
    })
  }
}

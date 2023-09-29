use storm_core::error::Result;
use storm_utils::traits::ReadExt;

use crate::parse::BoxedSize;
use crate::parse::ChunkType;
use crate::parse::ParseChunk;
use crate::types::Item;

// =============================================================================
// StarEdit Sprites (Doodads)
// =============================================================================

/// This section contains the doodad map of the level.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Dd2 {
  pub doodads: Box<[Dd2Data]>,
}

impl From<Dd2> for Item {
  #[inline]
  fn from(other: Dd2) -> Self {
    Self::Dd2(other)
  }
}

impl ParseChunk for Dd2 {
  const TYPE: ChunkType = ChunkType::Boxed(BoxedSize::Int(0x08));

  fn from_reader<R: ReadExt>(reader: &mut R, size: u32) -> Result<Self> {
    Ok(Self {
      doodads: Self::read_boxed(reader, size, Dd2Data::from_reader)?,
    })
  }
}

// =============================================================================
// Dd2 Data
// =============================================================================

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Dd2Data {
  /// Number of the doodad.
  ///
  /// Note: Size of the doodad is dependent on this.
  ///
  /// Note: Doodads are different for each tileset.
  pub id: u16,
  /// X coordinate of the doodad unit.
  pub x: u16,
  /// Y coordinate of the doodad unit.
  pub y: u16,
  /// Player number that owns the doodad.
  pub owner: u8,
  /// Enabled/Disabled flag.
  pub flag: u8,
}

impl Dd2Data {
  pub fn from_reader<R: ReadExt>(reader: &mut R) -> Result<Self> {
    Ok(Self {
      id: reader.read_u16_le()?,
      x: reader.read_u16_le()?,
      y: reader.read_u16_le()?,
      owner: reader.read_u8()?,
      flag: reader.read_u8()?,
    })
  }

  /// Returns `true` if the doodad is enabled.
  #[inline]
  pub const fn enabled(&self) -> bool {
    self.flag == 0x00
  }

  /// Returns `true` if the doodad is disabled.
  #[inline]
  pub const fn disabled(&self) -> bool {
    self.flag == 0x01
  }
}

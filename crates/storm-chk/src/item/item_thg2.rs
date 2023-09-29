use storm_core::error::Result;
use storm_utils::traits::ReadExt;

use crate::parse::BoxedSize;
use crate::parse::ChunkType;
use crate::parse::ParseChunk;
use crate::types::Item;

// =============================================================================
// StarCraft Sprites
// =============================================================================

/// Sprites, usually on doodads.
///
/// Required for all versions and all game types.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Thg2 {
  pub sprites: Box<[Thg2Data]>,
}

impl From<Thg2> for Item {
  #[inline]
  fn from(other: Thg2) -> Self {
    Self::Thg2(other)
  }
}

impl ParseChunk for Thg2 {
  const TYPE: ChunkType = ChunkType::Boxed(BoxedSize::Int(0x0A));

  fn from_reader<R: ReadExt>(reader: &mut R, size: u32) -> Result<Self> {
    Ok(Self {
      sprites: Self::read_boxed(reader, size, Thg2Data::from_reader)?,
    })
  }
}

// =============================================================================
// Thg2 Data
// =============================================================================

// TODO: bitflags
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Thg2Data {
  /// Unit/Sprite number of the sprite.
  id: u16,
  /// X coordinate of the doodad unit.
  x: u16,
  /// Y coordinate of the doodad unit.
  y: u16,
  /// Player number that owns the doodad.
  owner: u8,
  /// Unused (?)
  _padding: u8,
  /// Flags.
  flags: u16,
}

impl Thg2Data {
  pub fn from_reader<R: ReadExt>(reader: &mut R) -> Result<Self> {
    Ok(Self {
      id: reader.read_u16_le()?,
      x: reader.read_u16_le()?,
      y: reader.read_u16_le()?,
      owner: reader.read_u8()?,
      _padding: reader.read_u8()?,
      flags: reader.read_u16_le()?,
    })
  }
}

use storm_core::error::Result;
use storm_utils::traits::ReadExt;

use crate::parse::ChunkType;
use crate::parse::ParseChunk;
use crate::types::Item;

// =============================================================================
// Format Version
// =============================================================================

/// This section identifies the file format version.
///
/// Required for all versions and all game types.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Ver {
  pub version: u16,
}

impl Ver {
  /// Starcraft 1.00 (Original)
  pub const V1: u16 = 0x003B;

  /// Starcraft 1.04 (Hybrid)
  pub const V2: u16 = 0x003F;

  /// Starcraft Remastered (1.21) (Hybrid)
  pub const V3: u16 = 0x0040;

  /// Brood War 1.00 (1.04)
  pub const V4: u16 = 0x00CD;

  /// Starcraft Remastered (1.21) (Brood War)
  pub const V5: u16 = 0x00CE;

  // ===========================================================================
  // Unsupported Versions
  // ===========================================================================

  /// Warcraft II retail (.PUD)
  pub const UV1: u16 = 0x0011;

  /// Warcraft II Expansion (.PUD)
  pub const UV2: u16 = 0x0013;

  /// Starcraft Beta
  pub const UV3: u16 = 0x002F;

  /// Starcraft Prerelease
  pub const UV4: u16 = 0x0039;

  /// Brood War internal (map version 61)
  pub const UV5: u16 = 0x003D;

  /// Brood War internal (map version 75) (Broodwar Battle.net Beta)
  pub const UV6: u16 = 0x004B;

  /// Brood War internal (map version 201)
  pub const UV7: u16 = 0x00C9;

  /// Brood War internal (map version 203)
  pub const UV8: u16 = 0x00CB;
}

impl From<Ver> for Item {
  #[inline]
  fn from(other: Ver) -> Self {
    Self::Ver(other)
  }
}

impl ParseChunk for Ver {
  const TYPE: ChunkType = ChunkType::Sized(0x2);

  fn from_reader<R: ReadExt>(reader: &mut R, _size: u32) -> Result<Self> {
    Ok(Self {
      version: reader.read_u16_le()?,
    })
  }
}

use storm_core::error::Result;
use storm_utils::traits::ReadExt;

use crate::id::TechId;
use crate::parse::ChunkType;
use crate::parse::ParseChunk;
use crate::types::Item;

// =============================================================================
// Tech Settings
// =============================================================================

/// This section contains technology/special abilities settings.
///
/// Required for Vanilla and Hybrid (in Original mode). Not required for Melee.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Tecs {
  pub defaults: [u8; TechId::CLASSIC],
  pub min_cost: [u16; TechId::CLASSIC],
  pub gas_cost: [u16; TechId::CLASSIC],
  pub time_cost: [u16; TechId::CLASSIC],
  pub cast_cost: [u16; TechId::CLASSIC],
}

impl Tecs {
  /// Returns `true` if the `technology` uses default settings.
  #[inline]
  pub const fn defaults(&self, technology: TechId) -> bool {
    debug_assert!(technology.is_classic());
    self.defaults[technology as usize] == 0x01
  }

  /// Returns the mineral cost required to develop `technology`.
  #[inline]
  pub const fn min_cost(&self, technology: TechId) -> u16 {
    debug_assert!(technology.is_classic());
    self.min_cost[technology as usize]
  }

  /// Returns the gas cost required to develop `technology`.
  #[inline]
  pub const fn gas_cost(&self, technology: TechId) -> u16 {
    debug_assert!(technology.is_classic());
    self.gas_cost[technology as usize]
  }

  /// Returns the time required to develop `technology`.
  #[inline]
  pub const fn time_cost(&self, technology: TechId) -> u16 {
    debug_assert!(technology.is_classic());
    self.time_cost[technology as usize]
  }

  /// Returns the energy cost required to cast `technology`.
  #[inline]
  pub const fn cast_cost(&self, technology: TechId) -> u16 {
    debug_assert!(technology.is_classic());
    self.cast_cost[technology as usize]
  }
}

impl From<Tecs> for Item {
  #[inline]
  fn from(other: Tecs) -> Self {
    Self::Tecs(Box::new(other))
  }
}

impl ParseChunk for Tecs {
  const TYPE: ChunkType = ChunkType::Sized(0xD8);

  fn from_reader<R: ReadExt>(reader: &mut R, _size: u32) -> Result<Self> {
    Ok(Self {
      defaults: reader.read_array_u8()?,
      min_cost: reader.read_array_u16()?,
      gas_cost: reader.read_array_u16()?,
      time_cost: reader.read_array_u16()?,
      cast_cost: reader.read_array_u16()?,
    })
  }
}

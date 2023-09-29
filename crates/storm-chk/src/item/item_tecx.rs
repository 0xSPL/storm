use storm_core::error::Result;
use storm_utils::traits::ReadExt;

use crate::id::TechId;
use crate::parse::ChunkType;
use crate::parse::ParseChunk;
use crate::types::Item;

// =============================================================================
// Brood War Tech Settings
// =============================================================================

/// This section is identical to [`TECS`][super::Tecs] section except it uses
/// the Brood War set of 44 technologies instead of the original 24.
///
/// Required for Hybrid (in Expansion mode) and Brood War. Not required for Melee.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Tecx {
  pub defaults: [u8; TechId::BROOD_WAR],
  pub min_cost: [u16; TechId::BROOD_WAR],
  pub gas_cost: [u16; TechId::BROOD_WAR],
  pub time_cost: [u16; TechId::BROOD_WAR],
  pub cast_cost: [u16; TechId::BROOD_WAR],
}

impl Tecx {
  /// Returns `true` if the `technology` uses default settings.
  #[inline]
  pub const fn defaults(&self, technology: TechId) -> bool {
    self.defaults[technology as usize] == 0x01
  }

  /// Returns the mineral cost required to develop `technology`.
  #[inline]
  pub const fn min_cost(&self, technology: TechId) -> u16 {
    self.min_cost[technology as usize]
  }

  /// Returns the gas cost required to develop `technology`.
  #[inline]
  pub const fn gas_cost(&self, technology: TechId) -> u16 {
    self.gas_cost[technology as usize]
  }

  /// Returns the time required to develop `technology`.
  #[inline]
  pub const fn time_cost(&self, technology: TechId) -> u16 {
    self.time_cost[technology as usize]
  }

  /// Returns the energy cost required to cast `technology`.
  #[inline]
  pub const fn cast_cost(&self, technology: TechId) -> u16 {
    self.cast_cost[technology as usize]
  }
}

impl From<Tecx> for Item {
  #[inline]
  fn from(other: Tecx) -> Self {
    Self::Tecx(Box::new(other))
  }
}

impl ParseChunk for Tecx {
  const TYPE: ChunkType = ChunkType::Sized(0x18C);

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

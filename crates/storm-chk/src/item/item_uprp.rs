use storm_core::error::Result;
use storm_utils::bitflags;
use storm_utils::traits::ReadExt;

use crate::parse::ChunkType;
use crate::parse::ParseChunk;
use crate::types::Item;

// =============================================================================
// CUWP Slots
// =============================================================================

/// This section is used whenever the create units with properties trigger is
/// used. Since a slot has to be assigned to the action, this is where each slot
/// is designated.
///
/// Required for all versions. Not required for Melee.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Uprp {
  pub units: [UprpData; 0x40],
}

impl From<Uprp> for Item {
  #[inline]
  fn from(other: Uprp) -> Self {
    Self::Uprp(Box::new(other))
  }
}

impl ParseChunk for Uprp {
  const TYPE: ChunkType = ChunkType::Sized(0x500);

  fn from_reader<R: ReadExt>(reader: &mut R, _size: u32) -> Result<Self> {
    Ok(Self {
      units: Self::read_array(reader, UprpData::from_reader)?,
    })
  }
}

// =============================================================================
// Uprp Data
// =============================================================================

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct UprpData {
  /// Flags of special properties which can be applied to the unit.
  pub props_applied: UprpAppliedFlags,
  /// The properties which can be changed by the map maker.
  pub props_changed: UprpChangedFlags,
  /// Player number of owner.
  pub owner: u8,
  /// Hit points % (1-100).
  pub health: u8,
  /// Shield points % (1-100).
  pub shield: u8,
  /// Energy points % (1-100).
  pub energy: u8,
  /// Resource amount.
  pub resource: u32,
  /// Number of units in hangar.
  pub hangar: u16,
  /// Unit flags.
  pub flags: UprpUnitFlags,
  /// Unused (?)
  pub _padding: u32,
}

impl UprpData {
  pub fn from_reader<R: ReadExt>(reader: &mut R) -> Result<Self> {
    Ok(Self {
      props_applied: UprpAppliedFlags::from_value(reader.read_u16_le()?),
      props_changed: UprpChangedFlags::from_value(reader.read_u16_le()?),
      owner: reader.read_u8()?,
      health: reader.read_u8()?,
      shield: reader.read_u8()?,
      energy: reader.read_u8()?,
      resource: reader.read_u32_le()?,
      hangar: reader.read_u16_le()?,
      flags: UprpUnitFlags::from_value(reader.read_u16_le()?),
      _padding: reader.read_u32_le()?,
    })
  }
}

// =============================================================================
// Uprp Flags (Applied)
// =============================================================================

bitflags! {
  #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
  pub struct UprpAppliedFlags: u16 {
    /// Cloak bit is valid.
    const CLOAK = 0x0001;
    /// Burrowed bit is valid.
    const BURROWED = 0x0002;
    /// In transit bit is valid.
    const TRANSIT = 0x0004;
    /// Hallucinated bit is valid.
    const HALLUCINATED = 0x0008;
    /// Invincible bit is valid.
    const INVINCIBLE = 0x0010;
  }
}

// =============================================================================
// Uprp Flags (Changed)
// =============================================================================

bitflags! {
  #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
  pub struct UprpChangedFlags: u16 {
    /// Owner player is valid (unit is not neutral).
    const OWNER = 0x0001;
    /// HP is valid.
    const HEALTH = 0x0002;
    /// Shields is valid.
    const SHIELD = 0x0004;
    /// Energy is valid.
    const ENERGY = 0x0008;
    /// Resource amount is valid (unit is a resource).
    const RESOURCE = 0x0010;
    /// Amount in hanger is valid.
    const HANGAR = 0x0020;
  }
}

// =============================================================================
// Uprp Flags (Unit)
// =============================================================================

bitflags! {
  #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
  pub struct UprpUnitFlags: u16 {
    /// Unit is cloaked.
    const CLOAK = 0x0001;
    /// Unit is burrowed.
    const BURROWED = 0x0002;
    /// Building is in transit.
    const TRANSIT = 0x0004;
    /// Unit is hallucinated.
    const HALLUCINATED = 0x0008;
    /// Unit is invincible.
    const INVINCIBLE = 0x0010;
  }
}

use storm_core::error::Result;
use storm_utils::traits::ReadExt;

use crate::parse::BoxedSize;
use crate::parse::ChunkType;
use crate::parse::ParseChunk;
use crate::types::Item;

// =============================================================================
// Placed Units
// =============================================================================

/// This section contains all the pre-placed units on the map and their
/// properties.
///
/// Required for all versions and all game types.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Unit {
  pub units: Box<[UnitData]>,
}

impl From<Unit> for Item {
  #[inline]
  fn from(other: Unit) -> Self {
    Self::Unit(other)
  }
}

impl ParseChunk for Unit {
  const TYPE: ChunkType = ChunkType::Boxed(BoxedSize::Int(0x24));

  fn from_reader<R: ReadExt>(reader: &mut R, size: u32) -> Result<Self> {
    Ok(Self {
      units: Self::read_boxed(reader, size, UnitData::from_reader)?,
    })
  }
}

// =============================================================================
// Unit Data
// =============================================================================

// TODO: bitflags
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct UnitData {
  /// The unit's class instance.
  pub class: u32,
  /// X coordinate of unit.
  pub x: u16,
  /// Y coordinate of unit.
  pub y: u16,
  /// Unit Id.
  pub unit_id: u16,
  /// Type of relation to another building.
  pub unit_ty: u16,
  /// Flags of special properties which can be applied to the unit.
  pub props_applied: u16,
  /// The properties which can be changed by the map maker.
  pub props_changed: u16,
  /// Player number of owner.
  pub owner: u8,
  /// Hit points % (1-100).
  pub hp: u8,
  /// Shield points % (1-100).
  pub sp: u8,
  /// Energy points % (1-100).
  pub ep: u8,
  /// Resource amount.
  pub resource: u32,
  /// Number of units in hangar.
  pub hangar: u16,
  /// Unit state flags.
  pub state: u16,
  /// Unused (?)
  pub _padding: u32,
  /// Class instance of the unit to which this unit is related to.
  pub related: u32,
}

impl UnitData {
  pub fn from_reader<R: ReadExt>(reader: &mut R) -> Result<Self> {
    Ok(Self {
      class: reader.read_u32_le()?,
      x: reader.read_u16_le()?,
      y: reader.read_u16_le()?,
      unit_id: reader.read_u16_le()?,
      unit_ty: reader.read_u16_le()?,
      props_applied: reader.read_u16_le()?,
      props_changed: reader.read_u16_le()?,
      owner: reader.read_u8()?,
      hp: reader.read_u8()?,
      sp: reader.read_u8()?,
      ep: reader.read_u8()?,
      resource: reader.read_u32_le()?,
      hangar: reader.read_u16_le()?,
      state: reader.read_u16_le()?,
      _padding: reader.read_u32_le()?,
      related: reader.read_u32_le()?,
    })
  }
}

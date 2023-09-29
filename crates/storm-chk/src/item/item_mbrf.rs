use storm_core::error::Result;
use storm_utils::traits::ReadExt;

use crate::item::TrigData;
use crate::parse::BoxedSize;
use crate::parse::ChunkType;
use crate::parse::ParseChunk;
use crate::types::Item;

// =============================================================================
// Mission Briefings
// =============================================================================

/// This section contains all of the mission briefings shown by the players.
///
/// Required for all versions. Not required for Melee.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Mbrf {
  pub triggers: Box<[TrigData]>,
}

impl From<Mbrf> for Item {
  #[inline]
  fn from(other: Mbrf) -> Self {
    Self::Mbrf(other)
  }
}

impl ParseChunk for Mbrf {
  const TYPE: ChunkType = ChunkType::Boxed(BoxedSize::Int(0x960));

  fn from_reader<R: ReadExt>(reader: &mut R, size: u32) -> Result<Self> {
    Ok(Self {
      triggers: Self::read_boxed(reader, size, TrigData::from_reader)?,
    })
  }
}

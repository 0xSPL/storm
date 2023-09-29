use storm_core::error::Result;
use storm_utils::traits::ReadExt;

use crate::parse::BoxedSize;
use crate::parse::ChunkType;
use crate::parse::ParseChunk;
use crate::types::ChkString;
use crate::types::Item;

// =============================================================================
// String Data
// =============================================================================

/// This section contains all the strings in the map.
///
/// Required for all versions and all game types (or [`STR`][super::Str]).
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Strx {
  pub entries: u32,
  pub offsets: Box<[u32]>,
  pub content: Box<[u8]>,
}

impl Strx {
  #[inline]
  pub fn get(&self, index: usize) -> Option<&ChkString> {
    ChkString::read(index, &self.offsets, &self.content)
  }
}

impl From<Strx> for Item {
  #[inline]
  fn from(other: Strx) -> Self {
    Self::Strx(Box::new(other))
  }
}

impl ParseChunk for Strx {
  const TYPE: ChunkType = ChunkType::Boxed(BoxedSize::Dyn);

  fn from_reader<R: ReadExt>(reader: &mut R, size: u32) -> Result<Self> {
    let entries: u32 = reader.read_u32_le()?;
    let offsets: Box<[u32]> = reader.read_boxed_u32(entries as usize)?;

    // Set cursor according to previously parsed data
    let cursor: u32 = 4 + (entries << 2);

    // Adjust offsets accordingly
    //
    // TODO: Optimistic adjustment to avoid additional allocation
    let offsets: Box<[u32]> = offsets
      .iter()
      .copied()
      .map(|offset| offset - cursor)
      .collect();

    // Read all the string data
    let length: usize = (size - cursor) as usize;
    let content: Box<[u8]> = reader.read_boxed_u8(length)?;

    Ok(Self {
      entries,
      offsets,
      content,
    })
  }
}

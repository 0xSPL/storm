use core::mem::size_of;
use core::ops::Deref;
use storm_utils::traits::ParseContext;
use storm_utils::traits::ReadExt;
use storm_utils::utils::DigestMd5;

use crate::error::Error;
use crate::types::HeaderV1;
use crate::types::HeaderV2;
use crate::types::HeaderV3;

// =============================================================================
// Static Assertions
// =============================================================================

const_assert_size!(HeaderV4, 0xD0);

// =============================================================================
// Header V4
// =============================================================================

/// Archive Header (V4).
///
/// ## Layout
///
/// `0x00` = [`v3`][HeaderV3] \
/// `0x44` = `htable_size` \
/// `0x4C` = `btable_size` \
/// `0x54` = `hi_btable_size` \
/// `0x5C` = `het_table_size` \
/// `0x64` = `bet_table_size` \
/// `0x6C` = `raw_chunk_size` \
/// `0x70`..`0x80` = `md5_btable` \
/// `0x80`..`0x90` = `md5_htable` \
/// `0x90`..`0xA0` = `md5_hi_btable` \
/// `0xA0`..`0xB0` = `md5_bet_table` \
/// `0xB0`..`0xC0` = `md5_het_table` \
/// `0xC0`..`0xD0` = `md5_mpq_header`
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct HeaderV4 {
  /// Base header.
  pub v3: HeaderV3,
  /// Compressed size of the hash table.
  pub htable_size: u64,
  /// Compressed size of the block table.
  pub btable_size: u64,
  /// Compressed size of the hi-block table.
  pub hi_btable_size: u64,
  /// Compressed size of the HET block.
  pub het_table_size: u64,
  /// Compressed size of the BET block.
  pub bet_table_size: u64,
  /// Size of raw data to calculate MD5.
  pub raw_chunk_size: u32,
  /// MD5 of the block table before decryption.
  pub md5_btable: DigestMd5,
  /// MD5 of the hash table before decryption.
  pub md5_htable: DigestMd5,
  /// MD5 of the hi-block table.
  pub md5_hi_btable: DigestMd5,
  /// MD5 of the BET table before decryption.
  pub md5_bet_table: DigestMd5,
  /// MD5 of the HET table before decryption.
  pub md5_het_table: DigestMd5,
  /// MD5 of the MPQ header from signature to (including)
  /// [`md5_het_table`][HeaderV4::md5_het_table].
  pub md5_mpq_header: DigestMd5,
}

impl HeaderV4 {
  const RUST_PAD: usize = HeaderV3::RUST_PAD + 0x4; // TODO: This is kinda not ideal

  /// The size of a V4 header.
  pub const SIZE: usize = size_of::<Self>() - Self::RUST_PAD;

  /// Returns the MD5 digest of the header.
  pub fn digest(&self) -> DigestMd5 {
    DigestMd5::build(|hasher| {
      // V1 Fields
      hasher.update(self.magic);
      hasher.update(self.header_size.to_le_bytes());
      hasher.update(self.archive_size.to_le_bytes());
      hasher.update(self.format_version.to_le_bytes());
      hasher.update([self.sector_size_shift]);
      hasher.update([self._padding]);
      hasher.update(self.htable_offset.to_le_bytes());
      hasher.update(self.btable_offset.to_le_bytes());
      hasher.update(self.htable_entries.to_le_bytes());
      hasher.update(self.btable_entries.to_le_bytes());

      // V2 Fields
      hasher.update(self.hi_btable_offset.to_le_bytes());
      hasher.update(self.htable_offset_hi.to_le_bytes());
      hasher.update(self.btable_offset_hi.to_le_bytes());

      // V3 Fields
      hasher.update(self.archive_size_64.to_le_bytes());
      hasher.update(self.bet_table_position.to_le_bytes());
      hasher.update(self.het_table_position.to_le_bytes());

      // V4 Fields
      hasher.update(self.htable_size.to_le_bytes());
      hasher.update(self.btable_size.to_le_bytes());
      hasher.update(self.hi_btable_size.to_le_bytes());
      hasher.update(self.het_table_size.to_le_bytes());
      hasher.update(self.bet_table_size.to_le_bytes());
      hasher.update(self.raw_chunk_size.to_le_bytes());
      hasher.update(self.md5_btable);
      hasher.update(self.md5_htable);
      hasher.update(self.md5_hi_btable);
      hasher.update(self.md5_bet_table);
      hasher.update(self.md5_het_table);
    })
  }
}

impl Deref for HeaderV4 {
  type Target = HeaderV3;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.v3
  }
}

impl ParseContext<HeaderV1> for HeaderV4 {
  type Error = Error;

  /// Parse a V4 header from the given `reader`.
  fn from_reader<R: ReadExt + ?Sized>(
    context: HeaderV1,
    reader: &mut R,
  ) -> Result<Self, Self::Error> {
    HeaderV2::from_reader(context, reader).and_then(|context| reader.parse_context(context))
  }
}

impl ParseContext<HeaderV2> for HeaderV4 {
  type Error = Error;

  /// Parse a V4 header from the given `reader`.
  fn from_reader<R: ReadExt + ?Sized>(
    context: HeaderV2,
    reader: &mut R,
  ) -> Result<Self, Self::Error> {
    HeaderV3::from_reader(context, reader).and_then(|context| reader.parse_context(context))
  }
}

impl ParseContext<HeaderV3> for HeaderV4 {
  type Error = Error;

  /// Parse a V4 header from the given `reader`.
  fn from_reader<R: ReadExt + ?Sized>(
    context: HeaderV3,
    reader: &mut R,
  ) -> Result<Self, Self::Error> {
    Ok(Self {
      v3: context,
      htable_size: reader.read_u64_le()?,
      btable_size: reader.read_u64_le()?,
      hi_btable_size: reader.read_u64_le()?,
      het_table_size: reader.read_u64_le()?,
      bet_table_size: reader.read_u64_le()?,
      raw_chunk_size: reader.read_u32_le()?,
      md5_btable: reader.read_array_u8()?.into(),
      md5_htable: reader.read_array_u8()?.into(),
      md5_hi_btable: reader.read_array_u8()?.into(),
      md5_bet_table: reader.read_array_u8()?.into(),
      md5_het_table: reader.read_array_u8()?.into(),
      md5_mpq_header: reader.read_array_u8()?.into(),
    })
  }
}

only_serde! {
  use serde::__private::ser::FlatMapSerializer;
  use serde::ser::SerializeMap;
  use serde::Serialize;
  use serde::Serializer;

  impl Serialize for HeaderV4 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
      let mut state: S::SerializeMap = serializer.serialize_map(Some(27))?;
      self.v3.serialize(FlatMapSerializer(&mut state))?;
      state.serialize_entry("htable_size", &self.htable_size)?;
      state.serialize_entry("btable_size", &self.btable_size)?;
      state.serialize_entry("hi_btable_size", &self.hi_btable_size)?;
      state.serialize_entry("het_table_size", &self.het_table_size)?;
      state.serialize_entry("bet_table_size", &self.bet_table_size)?;
      state.serialize_entry("raw_chunk_size", &self.raw_chunk_size)?;
      state.serialize_entry("md5_btable", &self.md5_btable)?;
      state.serialize_entry("md5_htable", &self.md5_htable)?;
      state.serialize_entry("md5_hi_btable", &self.md5_hi_btable)?;
      state.serialize_entry("md5_bet_table", &self.md5_bet_table)?;
      state.serialize_entry("md5_het_table", &self.md5_het_table)?;
      state.serialize_entry("md5_mpq_header", &self.md5_mpq_header)?;
      state.end()
    }
  }
}

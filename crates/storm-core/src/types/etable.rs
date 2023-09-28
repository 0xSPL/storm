use core::cmp::Ordering;
use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::hash::Hash;
use core::hash::Hasher;
use core::marker::PhantomData;
use core::mem::size_of;
use storm_utils::traits::Parse;
use storm_utils::traits::ParseContext;
use storm_utils::traits::ReadExt;

use crate::error::Error;
use crate::error::ErrorKind;
use crate::types::Magic;

// =============================================================================
// Table Behaviour
// =============================================================================

trait TableMagic {
  const MAGIC: Magic;
}

pub enum MagicHET {}

impl TableMagic for MagicHET {
  const MAGIC: Magic = Magic::HET;
}

pub enum MagicBET {}

impl TableMagic for MagicBET {
  const MAGIC: Magic = Magic::BET;
}

// =============================================================================
// Static Assertions
// =============================================================================

const_assert_size!(HETHeader, 0x0C);
const_assert_size!(BETHeader, 0x0C);
const_assert_size!(ExtHTable, 0x2C);
const_assert_size!(ExtBTable, 0x58);

// =============================================================================
// Type Aliases
// =============================================================================

/// Header for HET table.
pub type HETHeader = ExtHeader<MagicHET>;

/// Header for BET table.
pub type BETHeader = ExtHeader<MagicBET>;

// =============================================================================
// Extended Table Header
// =============================================================================

/// Extended Table Header
///
/// ## Layout
///
/// `0x00` = `magic` \
/// `0x04` = `version` \
/// `0x08` = `data_size`
pub struct ExtHeader<M> {
  /// Table signature.
  pub magic: Magic,
  /// Table version.
  pub version: u32,
  /// Size of the contained table.
  pub data_size: u32,
  /// Magic signature marker.
  pub phantom: PhantomData<M>,
}

impl<M> ExtHeader<M> {
  /// The size of an extended table header.
  pub const SIZE: usize = size_of::<Self>();
}

impl<M> Clone for ExtHeader<M> {
  #[inline]
  fn clone(&self) -> Self {
    *self
  }
}

impl<M> Copy for ExtHeader<M> {}

impl<M> Debug for ExtHeader<M> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("ExtHeader")
      .field("magic", &self.magic)
      .field("version", &self.version)
      .field("data_size", &self.data_size)
      .finish()
  }
}

impl<M> Hash for ExtHeader<M> {
  #[inline]
  fn hash<H: Hasher>(&self, hasher: &mut H) {
    self.magic.hash(hasher);
    self.version.hash(hasher);
    self.data_size.hash(hasher);
  }
}

impl<M> PartialEq for ExtHeader<M> {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.magic == other.magic && self.version == other.version && self.data_size == other.data_size
  }
}

impl<M> Eq for ExtHeader<M> {}

impl<M> PartialOrd for ExtHeader<M> {
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl<M> Ord for ExtHeader<M> {
  #[inline]
  fn cmp(&self, other: &Self) -> Ordering {
    self
      .magic
      .cmp(&other.magic)
      .then_with(|| self.version.cmp(&other.version))
      .then_with(|| self.data_size.cmp(&other.data_size))
  }
}

impl<M> Parse for ExtHeader<M>
where
  M: TableMagic,
{
  type Error = Error;

  /// Parse an extended table header from the given `reader`.
  fn from_reader<R: ReadExt + ?Sized>(reader: &mut R) -> Result<Self, Self::Error> {
    let magic: Magic = reader.parse()?;

    // Ensure the header signature is correct
    if magic != M::MAGIC {
      return Err(Error::new(ErrorKind::InvalidMagic));
    }

    Ok(Self {
      magic,
      version: reader.read_u32_le()?,
      data_size: reader.read_u32_le()?,
      phantom: PhantomData,
    })
  }
}

// =============================================================================
// Extended Hash Table
// =============================================================================

/// Extended Hash Table.
///
/// ## Layout
///
/// `0x00` = `header` \
/// `0x0C` = `table_size` \
/// `0x10` = `entry_count` \
/// `0x14` = `total_count` \
/// `0x18` = `name_hash_bit_size` \
/// `0x1C` = `index_size_total` \
/// `0x20` = `index_size_extra` \
/// `0x24` = `index_size` \
/// `0x28` = `index_table_size`
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExtHTable {
  /// Base header.
  pub header: HETHeader,
  /// Size of the entire HET table, including the header (bytes).
  pub table_size: u32,
  /// Number of occupied entries in the HET table
  pub entry_count: u32,
  /// Total number of entries in the HET table.
  pub total_count: u32,
  /// Size of the name hash entry (bits).
  pub name_hash_bit_size: u32,
  /// Total size of the file index (bits).
  pub index_size_total: u32,
  /// Extra bits in the file index.
  pub index_size_extra: u32,
  /// Effective size of the file index (bits).
  pub index_size: u32,
  /// Size of the block index subtable (bytes).
  pub index_table_size: u32,
}

impl ExtHTable {
  /// The size of an extended hash table.
  pub const SIZE: usize = size_of::<Self>();
}

impl ParseContext<HETHeader> for ExtHTable {
  type Error = Error;

  /// Parse an extended hash table from the given `reader`.
  fn from_reader<R: ReadExt + ?Sized>(
    context: HETHeader,
    reader: &mut R,
  ) -> Result<Self, Self::Error> {
    debug_assert_eq!(context.magic, Magic::HET);

    Ok(Self {
      header: context,
      table_size: reader.read_u32_le()?,
      entry_count: reader.read_u32_le()?,
      total_count: reader.read_u32_le()?,
      name_hash_bit_size: reader.read_u32_le()?,
      index_size_total: reader.read_u32_le()?,
      index_size_extra: reader.read_u32_le()?,
      index_size: reader.read_u32_le()?,
      index_table_size: reader.read_u32_le()?,
    })
  }
}

// =============================================================================
// Extended Block Table
// =============================================================================

/// Extended Block Table.
///
/// ## Layout
///
/// `0x00` = `header` \
/// `0x0C` = `table_size` \
/// `0x10` = `entry_count` \
/// `0x18` = `entry_size` \
/// `0x1C` = `bi_file_position` \
/// `0x20` = `bi_file_size` \
/// `0x24` = `bi_comp_size` \
/// `0x28` = `bi_flag_index` \
/// `0x30` = `bc_file_position` \
/// `0x34` = `bc_file_size` \
/// `0x38` = `bc_comp_size` \
/// `0x3C` = `bc_flag_index` \
/// `0x44` = `bt_name_hash_2` \
/// `0x48` = `be_name_hash_2` \
/// `0x4C` = `bc_name_hash_2` \
/// `0x50` = `name_hash_array_size` \
/// `0x54` = `flag_count`
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExtBTable {
  /// Base header.
  pub header: BETHeader,
  /// Size of the entire BET table, including the header (bytes).
  pub table_size: u32,
  /// Number of entries in the BET table.
  pub entry_count: u32,
  /// Unused (?)
  pub _padding: u32,
  /// Size of a single table entry (bits).
  pub entry_size: u32,
  /// Bit index of the file position (within the entry record)
  pub bi_file_position: u32,
  /// Bit index of the file size (within the entry record)
  pub bi_file_size: u32,
  /// Bit index of the compressed size (within the entry record)
  pub bi_comp_size: u32,
  /// Bit index of the flag index (within the entry record)
  pub bi_flag_index: u32,
  /// Unused (?)
  pub _bi_padding: u32,
  /// Bit size of file position (in the entry record)
  pub bc_file_position: u32,
  /// Bit size of file size (in the entry record)
  pub bc_file_size: u32,
  /// Bit size of compressed file size (in the entry record)
  pub bc_comp_size: u32,
  /// Bit size of flags index (in the entry record)
  pub bc_flag_index: u32,
  /// Unused (?)
  pub _bc_padding: u32,
  /// Total bit size of the NameHash2.
  pub bt_name_hash_2: u32,
  /// Extra bits in the NameHash2.
  pub be_name_hash_2: u32,
  /// Effective size of NameHash2 (bits).
  pub bc_name_hash_2: u32,
  /// Size of NameHash2 table (bytes).
  pub name_hash_array_size: u32,
  /// Number of flags in the following array.
  pub flag_count: u32,
}

impl ExtBTable {
  /// The size of an extended block table.
  pub const SIZE: usize = size_of::<Self>();
}

impl ParseContext<BETHeader> for ExtBTable {
  type Error = Error;

  /// Parse an extended block table from the given `reader`.
  fn from_reader<R: ReadExt + ?Sized>(
    context: BETHeader,
    reader: &mut R,
  ) -> Result<Self, Self::Error> {
    debug_assert_eq!(context.magic, Magic::BET);

    Ok(Self {
      header: context,
      table_size: reader.read_u32_le()?,
      entry_count: reader.read_u32_le()?,
      _padding: reader.read_u32_le()?,
      entry_size: reader.read_u32_le()?,
      bi_file_position: reader.read_u32_le()?,
      bi_file_size: reader.read_u32_le()?,
      bi_comp_size: reader.read_u32_le()?,
      bi_flag_index: reader.read_u32_le()?,
      _bi_padding: reader.read_u32_le()?,
      bc_file_position: reader.read_u32_le()?,
      bc_file_size: reader.read_u32_le()?,
      bc_comp_size: reader.read_u32_le()?,
      bc_flag_index: reader.read_u32_le()?,
      _bc_padding: reader.read_u32_le()?,
      bt_name_hash_2: reader.read_u32_le()?,
      be_name_hash_2: reader.read_u32_le()?,
      bc_name_hash_2: reader.read_u32_le()?,
      name_hash_array_size: reader.read_u32_le()?,
      flag_count: reader.read_u32_le()?,
    })
  }
}

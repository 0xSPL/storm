use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::iter::Filter;
use core::mem::size_of;
use core::ops::Deref;
use core::slice::Iter;
use storm_utils::traits::Parse;
use storm_utils::traits::ReadExt;

use crate::error::Error;
use crate::traits::Table;
use crate::traits::TableEntry;

// =============================================================================
// Static Assertions
// =============================================================================

const_assert_size!(BTableEntry, 0x10);
const_assert_size!(HTableEntry, 0x10);

// =============================================================================
// Type Aliases
// =============================================================================

/// Hash Table.
pub type BTable = GenericTable<BTableEntry>;

/// Block Table.
pub type HTable = GenericTable<HTableEntry>;

/// Table Iterator.
pub type TableIter<'a, T> = Filter<Iter<'a, T>, fn(&&T) -> bool>;

// =============================================================================
// Generic Table
// =============================================================================

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct GenericTable<T> {
  pub(crate) data: Vec<T>,
}

impl<T> GenericTable<T> {
  /// Create a new `GenericTable<T>`.
  #[inline]
  pub const fn new() -> Self {
    Self { data: Vec::new() }
  }

  /// Create a new `GenericTable<T>` with the specified `capacity`.
  #[inline]
  pub fn with_capacity(capacity: usize) -> Self {
    Self {
      data: Vec::with_capacity(capacity),
    }
  }

  /// Returns the number of non-empty entries in the table.
  #[inline]
  pub fn count(&self) -> usize
  where
    T: TableEntry,
  {
    self.filter().count()
  }

  /// Returns an iterator over non-empty table entries.
  #[inline]
  pub fn filter(&self) -> TableIter<'_, T>
  where
    T: TableEntry,
  {
    self.data.iter().filter(|entry| !T::is_empty(entry))
  }

  /// Returns the size of the table data (bytes).
  #[inline]
  pub fn size(&self) -> usize
  where
    T: TableEntry,
  {
    self.data.len() * T::SIZE
  }
}

impl<T> Deref for GenericTable<T> {
  type Target = [T];

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.data
  }
}

impl<T> Debug for GenericTable<T>
where
  Self: Table,
  T: TableEntry + Debug,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.write_fmt(format_args!("{} ", Self::NAME))?;
    f.debug_list().entries(self.filter()).finish()
  }
}

// =============================================================================
// Hash Table Entry
// =============================================================================

/// Hash Table Entry.
///
/// ## Layout
///
/// `0x00` = `hash1` \
/// `0x04` = `hash2` \
/// `0x08` = `language` \
/// `0x0A` = `platform` \
/// `0x0C` = `position`
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct HTableEntry {
  /// The hash of the file path, using method A.
  pub hash1: u32,
  /// The hash of the file path, using method B.
  pub hash2: u32,
  /// The language of the file.
  ///
  /// This is a Windows `LANGID` data type, and uses the same values.
  pub language: u16,
  /// The platform the file is used for.
  ///
  /// 0 indicates the default platform. No other values have been observed.
  pub platform: u8,
  /// Unused (?)
  pub _padding: u8,
  /// If the hash table entry is valid, this is the index into the block table
  /// of the file. Otherwise, one of the following two values:
  ///
  /// `FFFFFFFF` - Hash table entry is empty, and has always been empty.
  ///              Terminates searches for a given file.
  ///
  /// `FFFFFFFE` - Hash table entry is empty, but was valid at some point
  ///              (deleted). Does not terminate searches for a given file.
  pub position: u32,
}

impl HTableEntry {
  /// The size of a hash table entry.
  pub const SIZE: usize = size_of::<Self>();

  /// Flag indicating this entry has always been empty.
  pub const EMPTY_FOREVER: u32 = 0xFFFFFFFF;

  /// Flag indicating this entry has been removed.
  pub const EMPTY_REMOVED: u32 = 0xFFFFFFFE;

  /// Returns `true` if the hash table entry is empty.
  pub const fn is_empty(&self) -> bool {
    self.position == Self::EMPTY_FOREVER || self.position == Self::EMPTY_REMOVED
  }
}

impl Parse for HTableEntry {
  type Error = Error;

  /// Parse a hash table entry from the given `reader`.
  fn from_reader<R: ReadExt>(reader: &mut R) -> Result<Self, Self::Error> {
    Ok(Self {
      hash1: reader.read_u32_le()?,
      hash2: reader.read_u32_le()?,
      language: reader.read_u16_le()?,
      platform: reader.read_u8()?,
      _padding: reader.read_u8()?,
      position: reader.read_u32_le()?,
    })
  }
}

// =============================================================================
// Block Table Entry
// =============================================================================

/// Block Table Entry.
///
/// ## Layout
///
/// `0x00` = `offset` \
/// `0x04` = `comp_size` \
/// `0x08` = `file_size` \
/// `0x0C` = `bitflags`
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BTableEntry {
  /// Offset of the beginning of the block, relative to the beginning of the archive.
  pub offset: u32,
  /// Size of the block in the archive.
  pub comp_size: u32,
  /// Size of the file data stored in the block.
  ///
  /// Only valid if the block is a file; otherwise meaningless, and should be 0.
  /// If the file is compressed, this is the size of the uncompressed file data.
  pub file_size: u32,
  /// Bit mask of the flags for the block.
  pub bitflags: BTableEntryFlags,
}

impl BTableEntry {
  /// The size of a block table entry.
  pub const SIZE: usize = size_of::<Self>();

  /// Returns `true` if the block table entry is empty (not a file).
  pub const fn is_empty(&self) -> bool {
    self.file_size == 0 && self.bitflags.is_empty()
  }

  /// Returns `true` if the `COMPRESSED` or `IMPLODED` flag is set.
  #[inline]
  pub const fn is_any_compression(&self) -> bool {
    self.is_compressed() || self.is_imploded()
  }

  /// Returns `true` if the `EXISTS` flag is set.
  #[inline]
  pub const fn is_exists(&self) -> bool {
    self.bitflags.contains(BTableEntryFlags::EXISTS)
  }

  /// Returns `true` if the `SIGNATURE` flag is set.
  #[inline]
  pub const fn is_signature(&self) -> bool {
    self.bitflags.contains(BTableEntryFlags::SIGNATURE)
  }

  /// Returns `true` if the `SECTOR_CRC` flag is set.
  #[inline]
  pub const fn is_sector_crc(&self) -> bool {
    self.bitflags.contains(BTableEntryFlags::SECTOR_CRC)
  }

  /// Returns `true` if the `DELETE_MARKER` flag is set.
  #[inline]
  pub const fn is_delete_marker(&self) -> bool {
    self.bitflags.contains(BTableEntryFlags::DELETE_MARKER)
  }

  /// Returns `true` if the `SINGLE_UNIT` flag is set.
  #[inline]
  pub const fn is_single_unit(&self) -> bool {
    self.bitflags.contains(BTableEntryFlags::SINGLE_UNIT)
  }

  /// Returns `true` if the `PATCH_FILE` flag is set.
  #[inline]
  pub const fn is_patch_file(&self) -> bool {
    self.bitflags.contains(BTableEntryFlags::PATCH_FILE)
  }

  /// Returns `true` if the `FIX_KEY` flag is set.
  #[inline]
  pub const fn is_fix_key(&self) -> bool {
    self.bitflags.contains(BTableEntryFlags::FIX_KEY)
  }

  /// Returns `true` if the `ENCRYPTED` flag is set.
  #[inline]
  pub const fn is_encrypted(&self) -> bool {
    self.bitflags.contains(BTableEntryFlags::ENCRYPTED)
  }

  /// Returns `true` if the `COMPRESSED` flag is set.
  #[inline]
  pub const fn is_compressed(&self) -> bool {
    self.bitflags.contains(BTableEntryFlags::COMPRESSED)
  }

  /// Returns `true` if the `IMPLODED` flag is set.
  #[inline]
  pub const fn is_imploded(&self) -> bool {
    self.bitflags.contains(BTableEntryFlags::IMPLODED)
  }
}

impl Parse for BTableEntry {
  type Error = Error;

  /// Parse a block table entry from the given `reader`.
  fn from_reader<R: ReadExt>(reader: &mut R) -> Result<Self, Self::Error> {
    Ok(Self {
      offset: reader.read_u32_le()?,
      comp_size: reader.read_u32_le()?,
      file_size: reader.read_u32_le()?,
      bitflags: BTableEntryFlags::from_value(reader.read_u32_le()?),
    })
  }
}

// =============================================================================
// Block Table Entry Flags
// =============================================================================

bitflags! {
  /// Block Table Entry Flags.
  #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
  pub struct BTableEntryFlags: u32 {
    /// Block is a file, and follows the file data format; otherwise, block is
    /// free space or unused.
    ///
    /// If the block is not a file, all other flags should be cleared,
    /// and [`file_size`][BTableEntry::file_size] should be 0.
    const EXISTS = 0x80000000;
    /// Present on `STANDARD.SNP\(signature)`.
    const SIGNATURE = 0x10000000;
    /// File has checksums for each sector.
    ///
    /// Ignored if file is not compressed or imploded.
    const SECTOR_CRC = 0x04000000;
    /// File is a deletion marker, indicating that the file no longer exists.
    ///
    /// This is used to allow patch archives to delete files present in
    /// lower-priority archives in the search chain.
    const DELETE_MARKER = 0x02000000;
    /// File is stored as a single unit, rather than split into sectors.
    const SINGLE_UNIT = 0x01000000;
    /// The file is a patch file.
    const PATCH_FILE = 0x00100000;
    /// The file's encryption key is adjusted by the block offset and file size.
    ///
    /// File must be encrypted.
    const FIX_KEY = 0x00020000;
    /// File is encrypted.
    const ENCRYPTED = 0x00010000;
    /// File is compressed. File cannot be imploded.
    const COMPRESSED = 0x00000200;
    /// File is imploded. File cannot be compressed.
    const IMPLODED = 0x00000100;
  }
}

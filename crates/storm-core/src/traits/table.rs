use storm_utils::traits::Parse;
use storm_utils::utils::DigestMd5;

use crate::consts::HASH_KEY_BT;
use crate::consts::HASH_KEY_HT;
use crate::error::Error;
use crate::types::BTable;
use crate::types::BTableEntry;
use crate::types::HTable;
use crate::types::HTableEntry;
use crate::types::HeaderV1;
use crate::types::HeaderV4;

// =============================================================================
// Table Entry Trait
// =============================================================================

pub trait TableEntry: Parse<Error = Error> {
  const SIZE: usize;

  fn is_empty(&self) -> bool;
}

// =============================================================================
// Table Trait
// =============================================================================

pub trait Table: Sized {
  /// The type of entry this table contains
  type Entry: TableEntry;

  /// The hash key of the table
  const HKEY: u32;

  /// The name of the table.
  const NAME: &'static str;

  const IDENT: &'static str;

  /// Returns the number of entries in the table
  fn entries(header: &HeaderV1) -> u32;

  /// Returns the offset of the table (relative to archive start)
  fn offset(header: &HeaderV1) -> u32;

  /// Returns the expected Md5 of the table.
  fn digest(header: &HeaderV4) -> DigestMd5;

  /// Returns the compressed size of the table.
  fn comp_size(header: &HeaderV4) -> u64;

  /// Creates a new table with room for `capacity` entries
  fn create(capacity: usize) -> Self;

  /// Adds a new entry to the table
  fn insert(&mut self, entry: Self::Entry);
}

// =============================================================================
// Hash Table Entry
// =============================================================================

impl TableEntry for HTableEntry {
  const SIZE: usize = Self::SIZE;

  #[inline]
  fn is_empty(&self) -> bool {
    Self::is_empty(self)
  }
}

// =============================================================================
// Block Table
// =============================================================================

impl TableEntry for BTableEntry {
  const SIZE: usize = Self::SIZE;

  #[inline]
  fn is_empty(&self) -> bool {
    Self::is_empty(self)
  }
}

// =============================================================================
// Hash Table
// =============================================================================

impl Table for HTable {
  type Entry = HTableEntry;

  const HKEY: u32 = HASH_KEY_HT;

  const NAME: &'static str = "hash table";

  const IDENT: &'static str = "HashTable";

  #[inline]
  fn entries(header: &HeaderV1) -> u32 {
    header.htable_entries
  }

  #[inline]
  fn offset(header: &HeaderV1) -> u32 {
    header.htable_offset
  }

  #[inline]
  fn digest(header: &HeaderV4) -> DigestMd5 {
    header.md5_htable
  }

  #[inline]
  fn comp_size(header: &HeaderV4) -> u64 {
    header.htable_size
  }

  #[inline]
  fn create(capacity: usize) -> Self {
    Self::with_capacity(capacity)
  }

  #[inline]
  fn insert(&mut self, entry: Self::Entry) {
    self.data.push(entry);
  }
}

// =============================================================================
// Block Table
// =============================================================================

impl Table for BTable {
  type Entry = BTableEntry;

  const HKEY: u32 = HASH_KEY_BT;

  const NAME: &'static str = "block table";

  const IDENT: &'static str = "BlockTable";

  #[inline]
  fn entries(header: &HeaderV1) -> u32 {
    header.btable_entries
  }

  #[inline]
  fn offset(header: &HeaderV1) -> u32 {
    header.btable_offset
  }

  #[inline]
  fn digest(header: &HeaderV4) -> DigestMd5 {
    header.md5_btable
  }

  #[inline]
  fn comp_size(header: &HeaderV4) -> u64 {
    header.btable_size
  }

  #[inline]
  fn create(capacity: usize) -> Self {
    Self::with_capacity(capacity)
  }

  #[inline]
  fn insert(&mut self, entry: Self::Entry) {
    self.data.push(entry);
  }
}

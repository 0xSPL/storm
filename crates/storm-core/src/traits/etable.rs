use storm_utils::traits::Parse;
use storm_utils::traits::ParseContext;
use storm_utils::utils::DigestMd5;

use crate::consts::HASH_KEY_BT;
use crate::consts::HASH_KEY_HT;
use crate::error::Error;
use crate::types::BETHeader;
use crate::types::ExtBTable;
use crate::types::ExtHTable;
use crate::types::HETHeader;
use crate::types::HeaderV3;
use crate::types::HeaderV4;

// =============================================================================
// Extended Table Trait
// =============================================================================

pub trait ExtTable: ParseContext<Self::Header, Error = Error> {
  type Header: ExtTableHeader;

  /// The hash key of the table
  const HKEY: u32;

  /// The name of the table.
  const NAME: &'static str;

  /// Returns the offset of the table (relative to archive start)
  fn offset(header: &HeaderV3) -> u64;

  /// Returns the expected Md5 of the table.
  fn digest(header: &HeaderV4) -> DigestMd5;

  /// Returns the compressed size of the table.
  fn comp_size(header: &HeaderV4) -> u64;
}

// =============================================================================
// Extended Table Header Trait
// =============================================================================

pub trait ExtTableHeader: Parse<Error = Error> {
  const SIZE: usize;
}

// =============================================================================
// HET Table Header
// =============================================================================

impl ExtTableHeader for HETHeader {
  const SIZE: usize = Self::SIZE;
}

// =============================================================================
// BET Table Header
// =============================================================================

impl ExtTableHeader for BETHeader {
  const SIZE: usize = Self::SIZE;
}

// =============================================================================
// HET Table
// =============================================================================

impl ExtTable for ExtHTable {
  type Header = HETHeader;

  const HKEY: u32 = HASH_KEY_HT;

  const NAME: &'static str = "HET table";

  #[inline]
  fn offset(header: &HeaderV3) -> u64 {
    header.het_table_position
  }

  #[inline]
  fn digest(header: &HeaderV4) -> DigestMd5 {
    header.md5_het_table
  }

  #[inline]
  fn comp_size(header: &HeaderV4) -> u64 {
    header.het_table_size
  }
}

// =============================================================================
// BET Table
// =============================================================================

impl ExtTable for ExtBTable {
  type Header = BETHeader;

  const HKEY: u32 = HASH_KEY_BT;

  const NAME: &'static str = "BET table";

  #[inline]
  fn offset(header: &HeaderV3) -> u64 {
    header.bet_table_position
  }

  #[inline]
  fn digest(header: &HeaderV4) -> DigestMd5 {
    header.md5_bet_table
  }

  #[inline]
  fn comp_size(header: &HeaderV4) -> u64 {
    header.bet_table_size
  }
}

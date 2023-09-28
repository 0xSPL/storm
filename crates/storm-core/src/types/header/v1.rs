use core::mem::size_of;
use storm_utils::traits::ParseContext;
use storm_utils::traits::ReadExt;

use crate::consts::BT_MASK;
use crate::error::Error;
use crate::types::Magic;

// =============================================================================
// Static Assertions
// =============================================================================

const_assert_size!(HeaderV1, 0x20);

// =============================================================================
// Header V1
// =============================================================================

/// Archive Header (V1).
///
/// ## Layout
///
/// `0x00` = `magic` \
/// `0x04` = `header_size` \
/// `0x08` = `archive_size` \
/// `0x0C` = `format_version` \
/// `0x0E` = `sector_size_shift` \
/// `0x10` = `htable_offset` \
/// `0x14` = `btable_offset` \
/// `0x18` = `htable_entries` \
/// `0x1C` = `btable_entries`
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct HeaderV1 {
  /// Indicates that the file is a MoPaQ archive.
  pub magic: Magic,
  /// Size of the archive header.
  pub header_size: u32,
  /// Size of the whole archive, including the header.
  ///
  /// This field is deprecated in the Burning Crusade MoPaQ format, and the size
  /// of the archive is calculated as the size from the beginning of the archive
  /// to the end of the hash table, block table, or extended block table
  /// (whichever is largest).
  pub archive_size: u32,
  /// MoPaQ format version.
  pub format_version: u16,
  /// Power of two exponent specifying the number of 512-byte disk sectors in
  /// each logical sector in the archive.
  pub sector_size_shift: u8,
  /// Unused (?)
  pub _padding: u8,
  /// Offset to the beginning of the hash table, relative to the beginning of
  /// the archive.
  pub htable_offset: u32,
  /// Offset to the beginning of the block table, relative to the beginning of
  /// the archive.
  pub btable_offset: u32,
  /// Number of entries in the hash table.
  ///
  /// Must be a power of two, and must be less than 2^16 for the original
  /// MoPaQ format, or less than 2^20 for the Burning Crusade format.
  pub htable_entries: u32,
  /// Number of entries in the block table.
  pub btable_entries: u32,
}

impl HeaderV1 {
  /// The size of a V1 header.
  pub const SIZE: usize = size_of::<Self>();
}

impl ParseContext<Magic> for HeaderV1 {
  type Error = Error;

  /// Parse a V1 header from the given `reader`.
  fn from_reader<R: ReadExt + ?Sized>(context: Magic, reader: &mut R) -> Result<Self, Self::Error> {
    debug_assert_eq!(context, Magic::ID);

    Ok(Self {
      magic: context,
      header_size: reader.read_u32_le()?,
      archive_size: reader.read_u32_le()?,
      format_version: reader.read_u16_le()?,
      sector_size_shift: reader.read_u8()?,
      _padding: reader.read_u8()?,
      htable_offset: reader.read_u32_le()?,
      btable_offset: reader.read_u32_le()?,
      htable_entries: reader.read_u32_le()? & BT_MASK,
      btable_entries: reader.read_u32_le()? & BT_MASK,
    })
  }
}

use core::mem::size_of;
use storm_utils::traits::ParseContext;
use storm_utils::traits::ReadExt;

use crate::error::Error;
use crate::types::Magic;

// =============================================================================
// Static Assertions
// =============================================================================

const_assert_size!(UserData, 0x10);

// =============================================================================
// User Data
// =============================================================================

/// User Data Block.
///
/// ## Layout
///
/// `0x00` = `magic` \
/// `0x04` = `udata_size` \
/// `0x08` = `header_offset` \
/// `0x0C` = `udata_header_size`
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct UserData {
  /// Indicates that this is a shunt block.
  pub magic: Magic,
  /// The number of bytes that have been allocated in this archive for user data.
  ///
  /// This does not need to be the exact size of the data itself, but merely the
  /// maximum amount of data which may be stored in this archive.
  pub udata_size: u32,
  /// The offset in the file at which to continue the search for the archive
  /// header.
  pub header_offset: u32,
  /// ~The block to store user data in.~
  ///
  /// The size of the user data header.
  pub udata_header_size: u32,
}

impl UserData {
  /// The size of a user data block.
  pub const SIZE: usize = size_of::<Self>();
}

impl ParseContext<Magic> for UserData {
  type Error = Error;

  /// Parse a user data block from the given `reader`.
  fn from_reader<R: ReadExt>(context: Magic, reader: &mut R) -> Result<Self, Self::Error> {
    debug_assert_eq!(context, Magic::UD);

    Ok(Self {
      magic: context,
      udata_size: reader.read_u32_le()?,
      header_offset: reader.read_u32_le()?,
      udata_header_size: reader.read_u32_le()?,
    })
  }
}

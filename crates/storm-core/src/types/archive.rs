use std::path::Path;

use crate::error::Result;
use crate::extract::find_file;
use crate::extract::FilePtr;
use crate::parse::read_archive;
use crate::parse::Handle;
use crate::types::BTable;
use crate::types::ExtBTable;
use crate::types::ExtHTable;
use crate::types::HTable;
use crate::types::Header;
use crate::types::Signature;
use crate::types::UserData;

// =============================================================================
// Archive
// =============================================================================

/// MoPaQ Archive
#[derive(Debug)]
pub struct Archive {
  /// Archive file handle.
  pub handle: Handle,
  /// The offset in the file where the archive was found.
  pub offset: u64,
  /// Archive Header.
  pub header: Header,
  /// User Data.
  pub udata: Option<UserData>,
  /// Hash Table.
  pub htable: HTable,
  /// Block Table.
  pub btable: BTable,
  /// Extended Hash Table.
  pub ext_htable: Option<ExtHTable>,
  /// Extended Block Table.
  pub ext_btable: Option<ExtBTable>,
  /// Strong Digital Signature.
  pub signature: Option<Signature>,
}

impl Archive {
  /// Returns a reference to the MPQ header.
  #[inline]
  pub const fn header(&self) -> &Header {
    &self.header
  }

  /// Returns a reference to the MPQ user data.
  #[inline]
  pub const fn udata(&self) -> Option<&UserData> {
    self.udata.as_ref()
  }

  /// Returns a reference to the MPQ hash table.
  #[inline]
  pub const fn htable(&self) -> &HTable {
    &self.htable
  }

  /// Returns a reference to the MPQ block table.
  #[inline]
  pub const fn btable(&self) -> &BTable {
    &self.btable
  }

  /// Returns a reference to the MPQ extended hash table.
  #[inline]
  pub const fn ext_htable(&self) -> Option<&ExtHTable> {
    self.ext_htable.as_ref()
  }

  /// Returns a reference to the MPQ extended block table.
  #[inline]
  pub const fn ext_btable(&self) -> Option<&ExtBTable> {
    self.ext_btable.as_ref()
  }

  /// Returns a reference to the strong digital signature.
  #[inline]
  pub const fn signature(&self) -> Option<&Signature> {
    self.signature.as_ref()
  }

  /// Returns the size of each logical sector in the archive.
  #[inline]
  pub fn sector_size(&self) -> u32 {
    0x200 << self.header.sector_size_shift
  }

  /// Parse an archive from the file at the given `path`.
  #[inline]
  pub fn open<P>(path: &P) -> Result<Self>
  where
    P: AsRef<Path> + ?Sized,
  {
    read_archive(path)
  }

  /// Search the archive for a file with the given `name`.
  #[inline]
  pub fn find_file<'a>(&'a self, name: &'a str) -> Result<FilePtr<'a>> {
    find_file(self, name)
  }
}

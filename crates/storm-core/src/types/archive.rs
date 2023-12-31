use std::path::Path;

use crate::error::Result;
use crate::extract::find_file;
use crate::extract::FilePtr;
use crate::parse::read_archive;
use crate::parse::Handle;
use crate::types::AttrFile;
use crate::types::BTable;
use crate::types::ExtBTable;
use crate::types::ExtHTable;
use crate::types::File;
use crate::types::HTable;
use crate::types::Header;
use crate::types::ListFile;
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

  /// Find and load a file with the given `name`.
  pub fn load_file(&self, name: &str) -> Result<File> {
    self.find_file(name).and_then(FilePtr::read)
  }

  /// Load the `(listfile)` from the archive.
  pub fn load_listfile(&self) -> Result<ListFile> {
    self.find_file("(listfile)").and_then(ListFile::try_from)
  }

  /// Load the `(attributes)` from the archive.
  pub fn load_attributes(&self) -> Result<AttrFile> {
    self.find_file("(attributes)").and_then(AttrFile::try_from)
  }

  /// Load the `(signature)` from the archive.
  pub fn load_signature(&self) -> Result<File> {
    self.load_file("(signature)")
  }

  /// Load the `(user data)` from the archive.
  pub fn load_user_data(&self) -> Result<File> {
    self.load_file("(user data)")
  }
}

only_serde! {
  use serde::ser::SerializeStruct;
  use serde::Serialize;
  use serde::Serializer;

  impl Serialize for Archive {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
      let mut state: S::SerializeStruct = serializer.serialize_struct("Archive", 9)?;
      state.serialize_field("handle", &self.handle)?;
      state.serialize_field("offset", &self.offset)?;
      state.serialize_field("header", &self.header)?;
      state.serialize_field("udata", &self.udata)?;
      state.serialize_field("htable", &self.htable)?;
      state.serialize_field("btable", &self.btable)?;
      state.serialize_field("ext_htable", &self.ext_htable)?;
      state.serialize_field("ext_btable", &self.ext_btable)?;
      state.serialize_field("signature", &self.signature)?;
      state.end()
    }
  }
}

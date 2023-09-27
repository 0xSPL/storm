use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::num::NonZeroU16;
use core::num::NonZeroU8;

use crate::consts::BT_MASK;
use crate::error::Error;
use crate::error::ErrorKind;
use crate::error::Result;
use crate::types::Archive;
use crate::types::BTable;
use crate::types::BTableEntry;
use crate::types::HTable;
use crate::types::HTableEntry;
use crate::types::Header;
use crate::utils;
use crate::utils::HashType;

/// Search the archive for a file matching the given `query`.
pub fn find_file<'a, Q>(archive: &'a Archive, query: Q) -> Result<FilePtr<'a>>
where
  Q: Into<Query<'a>>,
{
  if let Some(file) = search(archive, query.into()) {
    // Check if the file really exists
    if file.btentry.is_exists() {
      if is_impossibly_large(archive, &file) {
        return Err(Error::new(ErrorKind::FileCorruptData));
      }

      return Ok(file);
    }
  }

  Err(Error::new(ErrorKind::FileDataMissing))
}

fn search<'a>(archive: &'a Archive, query: Query<'a>) -> Option<FilePtr<'a>> {
  let htable: &HTable = archive.htable();
  let btable: &BTable = archive.btable();

  // First check the hash table
  if !htable.is_empty() {
    if let Some(entry) = search_htable(archive, query) {
      return Some(FilePtr {
        query,
        archive,
        btentry: &btable[entry.position as usize],
      });
    }
  }

  None
}

fn search_htable<'a>(archive: &'a Archive, query: Query<'_>) -> Option<&'a HTableEntry> {
  // Keep a reference to the best possible candidate
  let mut best: Option<&HTableEntry> = None;

  let header: &Header = archive.header();
  let htable: &HTable = archive.htable();

  let hash1: u32 = utils::hash(query.filename, HashType::NameA);
  let hash2: u32 = utils::hash(query.filename, HashType::NameB);
  let index: u32 = utils::hash(query.filename, HashType::Table);
  let start: usize = index as usize & (htable.len() - 1);

  for entry in htable[start..].iter().chain(htable[..start].iter()) {
    // Check both hashes and block position for a matching entry
    if is_match(header, entry, hash1, hash2) {
      // Check if the entry matches language and platform, only if values given
      if query.is_exact(entry) {
        return Some(entry);
      }

      // Check if the entry matches language and platform
      if query.is_language(entry) && query.is_platform(entry) {
        best = Some(entry);
      }
    }

    // Check if the entry has always been empty - terminate search if so
    if entry.position == HTableEntry::EMPTY_FOREVER {
      break;
    }
  }

  best
}

const fn is_match(header: &Header, entry: &HTableEntry, hash1: u32, hash2: u32) -> bool {
  if entry.position & BT_MASK >= header.v1().btable_entries {
    return false;
  }

  entry.hash1 == hash1 && entry.hash2 == hash2
}

// MPQ Protector Guard
//
// Check if the contained file claims to be larger than the actual file
fn is_impossibly_large(archive: &Archive, file: &FilePtr<'_>) -> bool {
  !file.btentry.is_any_compression() && u64::from(file.btentry.file_size) > archive.handle.size()
}

// =============================================================================
// File Pointer
// =============================================================================

#[derive(Clone, Copy)]
pub struct FilePtr<'a> {
  pub(crate) query: Query<'a>,
  pub(crate) archive: &'a Archive,
  pub(crate) btentry: &'a BTableEntry,
}

impl FilePtr<'_> {
  // Returns the source offset of the file.
  pub(crate) fn offset(&self) -> u64 {
    self.archive.offset + u64::from(self.btentry.offset)
  }

  // Computes the encryption key of the file.
  pub(crate) fn encryption_key(&self) -> u32 {
    let mut key: u32 = 0;

    if self.btentry.is_encrypted() {
      // Find the file name part of the path
      let name: &str = match self.query.filename.rsplit(['\\', '/']).next() {
        Some(name) => name,
        None => panic!("Failed to extract file name"),
      };

      // Hash the name to get the base key
      key = utils::hash(name, HashType::File);

      // Offset-adjust the key if necessary
      if self.btentry.is_fix_key() {
        key = (key + self.btentry.offset) ^ self.btentry.file_size;
      }
    }

    key
  }
}

impl Debug for FilePtr<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("FilePtr")
      .field("query", &self.query)
      .field("btentry", &self.btentry)
      .finish_non_exhaustive()
  }
}

// =============================================================================
// Query
// =============================================================================

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Query<'a> {
  filename: &'a str,
  language: Option<NonZeroU16>,
  platform: Option<NonZeroU8>,
}

impl<'a> Query<'a> {
  /// Create a new `Query`.
  pub const fn new(filename: &'a str) -> Self {
    Self {
      filename,
      language: None,
      platform: None,
    }
  }

  /// Set the query language.
  pub fn set_language(&mut self, language: u16) {
    self.language = NonZeroU16::new(language);
  }

  /// Set the query platform.
  #[inline]
  pub fn set_platform(&mut self, platform: u8) {
    self.platform = NonZeroU8::new(platform);
  }

  #[inline]
  const fn is_exact(&self, entry: &HTableEntry) -> bool {
    if let (Some(language), Some(platform)) = (self.language, self.platform) {
      entry.language == language.get() && entry.platform == platform.get()
    } else {
      false
    }
  }

  #[inline]
  const fn is_language(&self, entry: &HTableEntry) -> bool {
    if let Some(language) = self.language {
      entry.language == language.get()
    } else {
      true
    }
  }

  #[inline]
  fn is_platform(&self, entry: &HTableEntry) -> bool {
    if let Some(platform) = self.platform {
      entry.platform == platform.get()
    } else {
      true
    }
  }
}

impl<'a> From<&'a str> for Query<'a> {
  #[inline]
  fn from(other: &'a str) -> Self {
    Self::new(other)
  }
}

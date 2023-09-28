use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Error as FmtError;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::iter::Filter;
use core::ops::Deref;
use core::slice::Split;
use core::str::from_utf8;

use crate::error::Result;
use crate::types::File;

// =============================================================================
// ListFile
// =============================================================================

/// Contains the paths of files in the archive.
#[derive(Clone, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct ListFile(File);

impl ListFile {
  /// Create a new `ListFile`.
  #[inline]
  pub const fn new(file: File) -> Self {
    Self(file)
  }

  /// Returns the number of entries in the `ListFile`.
  #[inline]
  pub fn len(&self) -> usize {
    self.iter().count()
  }

  /// Returns `true` if the `ListFile` has no entries.
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  /// Returns an iterator over the entries in the `ListFile`.
  #[inline]
  pub fn iter(&self) -> ListIter<'_> {
    ListIter::new(self)
  }
}

impl Debug for ListFile {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.write_str("ListFile ")?;
    f.debug_list().entries(self.iter()).finish()
  }
}

impl Deref for ListFile {
  type Target = File;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<'a> IntoIterator for &'a ListFile {
  type Item = ListEntry<'a>;
  type IntoIter = ListIter<'a>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a> IntoIterator for &'a mut ListFile {
  type Item = ListEntry<'a>;
  type IntoIter = ListIter<'a>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

// =============================================================================
// ListFile Entry
// =============================================================================

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct ListEntry<'a>(&'a [u8]);

impl<'a> ListEntry<'a> {
  /// Create a new `ListEntry`.
  #[inline]
  pub const fn new(data: &'a [u8]) -> Self {
    Self(data)
  }

  /// Convert the entry to a UTF-8 string slice.
  #[inline]
  pub fn as_utf8(&self) -> Result<&'a str> {
    from_utf8(self.0).map_err(Into::into)
  }
}

impl Debug for ListEntry<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{:?}", self.as_utf8().map_err(|_| FmtError)?)
  }
}

impl Display for ListEntry<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}", self.as_utf8().map_err(|_| FmtError)?)
  }
}

// =============================================================================
// ListFile Iterator
// =============================================================================

type InnerSplit<'a> = Split<'a, u8, fn(&u8) -> bool>;
type InnerFilter<'a> = Filter<InnerSplit<'a>, fn(&&'a [u8]) -> bool>;

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct ListIter<'a>(InnerFilter<'a>);

impl<'a> ListIter<'a> {
  #[inline]
  fn new(file: &'a ListFile) -> Self {
    let base: InnerSplit<'a> = file.split(Self::__splitter);
    let iter: InnerFilter<'a> = base.filter(Self::__filter);

    Self(iter)
  }

  #[inline]
  const fn __splitter(ch: &u8) -> bool {
    matches!(ch, 0x3B | 0x0D | 0x0A) // ';' | '\r' | '\n'
  }

  #[inline]
  const fn __filter(data: &&[u8]) -> bool {
    !data.is_empty()
  }
}

impl<'a> Iterator for ListIter<'a> {
  type Item = ListEntry<'a>;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    self.0.next().map(ListEntry::new)
  }
}

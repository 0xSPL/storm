use std::path::Path;
use storm_utils::traits::Parse;
use storm_utils::traits::ParseContext;
use storm_utils::traits::ReadExt;
use storm_utils::traits::SeekExt;
use storm_utils::utils::DigestMd5;

use crate::consts::HJUMP;
use crate::consts::MAGIC_ID;
use crate::consts::MAGIC_SIGN;
use crate::consts::MAGIC_UD;
use crate::error::Error;
use crate::error::ErrorKind;
use crate::error::Result;
use crate::parse::Handle;
use crate::traits::ExtTable;
use crate::traits::ExtTableHeader;
use crate::traits::Table;
use crate::traits::TableEntry;
use crate::types::Archive;
use crate::types::BTable;
use crate::types::ExtBTable;
use crate::types::ExtHTable;
use crate::types::HTable;
use crate::types::Header;
use crate::types::HeaderV4;
use crate::types::Magic;
use crate::types::Signature;
use crate::types::UserData;
use crate::utils;

/// Parse an archive from the file at the given `path`.
pub fn read_archive<P>(path: &P) -> Result<Archive>
where
  P: AsRef<Path> + ?Sized,
{
  Handle::new(path)
    .map(Buffer::new)
    .and_then(Buffer::parse_archive)
}

/// Parse an archive header from the file at the given `path`.
pub fn read_header<P>(path: &P) -> Result<Header>
where
  P: AsRef<Path> + ?Sized,
{
  Handle::new(path)
    .map(Buffer::new)
    .and_then(Buffer::parse_header)
}

// =============================================================================
// Parse Buffer
// =============================================================================

#[derive(Debug)]
struct Buffer {
  offset: u64,             // current position in the reader
  reader: Handle,          // file handle
  buffer: Vec<u8>,         // buffer re-used to avoid extra allocations
  magic: [u8; 4],          // "magic"
  found: bool,             // true if the archive header was found
  udata: Option<UserData>, // additional user-data
}

impl Buffer {
  const BUFFER: usize = 0x200;

  #[inline]
  fn new(reader: Handle) -> Self {
    Self {
      offset: 0,
      reader,
      buffer: vec![0; Self::BUFFER],
      magic: [0; 4],
      found: false,
      udata: None,
    }
  }

  #[inline]
  fn seek(&self, offset: u64) -> u64 {
    self.offset + offset
  }

  #[inline]
  fn parse_header(mut self) -> Result<Header> {
    self.header()
  }

  fn parse_archive(mut self) -> Result<Archive> {
    let header: Header = self.header()?;
    let htable: HTable = self.table(&header)?;
    let btable: BTable = self.table(&header)?;
    let signature: Option<Signature> = self.signature(&header)?;

    let mut ext_htable: Option<ExtHTable> = None;
    let mut ext_btable: Option<ExtBTable> = None;

    if let Some(header) = header.v4() {
      if header.het_table_position != 0 {
        ext_htable = Some(self.etable(header)?);
      }

      if header.bet_table_position != 0 {
        ext_btable = Some(self.etable(header)?);
      }
    }

    Ok(Archive {
      offset: self.offset,
      header,
      udata: self.udata.take(),
      htable,
      btable,
      ext_htable,
      ext_btable,
      signature,
    })
  }

  fn header(&mut self) -> Result<Header> {
    let header: Header = self.scan_header()?;

    // Verify MD5 if we have a V4 header.
    if let Some(header) = header.v4() {
      if header.digest() != header.md5_mpq_header {
        return Err(Error::new(ErrorKind::InvalidMd5("header")));
      }
    }

    Ok(header)
  }

  fn scan_header(&mut self) -> Result<Header> {
    // Loop until we find the header
    loop {
      // If we haven't found the correct offset, keep seeking.
      if !self.found {
        self.reader.seek_start(self.seek(0))?;
        self.reader.read_bytes(&mut self.magic)?;
      }

      if self.found || self.magic == MAGIC_ID {
        return Header::from_reader(Magic::ID, &mut self.reader);
      } else if self.magic == MAGIC_UD {
        if self.found {
          eprintln!("[warning]: Multiple User Data Blocks");
        }

        // Parse user data block
        let udata: UserData = UserData::from_reader(Magic::UD, &mut self.reader)?;

        // Sanity Checks
        assert!(udata.udata_header_size <= udata.udata_size);
        assert!(udata.udata_size <= udata.header_offset);

        // Increment the file offset accordingly
        self.offset += u64::from(udata.header_offset);

        // Move the cursor to the new offset and check header "magic"
        self.reader.seek_start(self.seek(0))?;
        self.reader.read_bytes(&mut self.magic)?;

        // If it looks like a header, flag it and parse on next iteration
        if self.magic == MAGIC_ID {
          self.found = true;
          self.udata = Some(udata);
        } else {
          return Err(Error::new(ErrorKind::FileCorruptData));
        }
      } else {
        // Didn't find a matching header - skip to the next sector
        self.offset += HJUMP;
      }
    }
  }

  fn table<T: Table>(&mut self, header: &Header) -> Result<T> {
    let entries: usize = T::entries(header) as usize;
    let capacity: usize = entries * T::Entry::SIZE;
    let position: u64 = self.seek(u64::from(T::offset(header)));

    // Clear the buffer and ensure we have enough capacity
    self.buffer.clear();
    self.buffer.resize(capacity, 0);

    // We only need to operate with a slice of the buffer
    let window: &mut [u8] = &mut self.buffer[..capacity];

    // Seek to the table and read into the buffer
    self.reader.seek_start(position)?;
    self.reader.read_bytes(window)?;

    if let Some(header) = header.v4() {
      // Verify table size if we have a V4 header.
      if window.len() as u64 != T::comp_size(header) {
        return Err(Error::new(ErrorKind::InvalidLen(T::NAME)));
      }

      // Verify MD5 if we have a V4 header.
      if DigestMd5::new(window) != T::digest(header) {
        return Err(Error::new(ErrorKind::InvalidMd5(T::NAME)));
      }
    }

    // Decrypt the table
    utils::decrypt(window, T::HKEY)?;

    // Allocate the table
    let mut table: T = T::create(capacity);

    // Read all entries in the table
    for index in 0..entries {
      let slice: &[u8] = &window[index * T::Entry::SIZE..][..T::Entry::SIZE];
      let entry: T::Entry = T::Entry::from_slice(slice)?;

      table.insert(entry);
    }

    Ok(table)
  }

  fn etable<T: ExtTable>(&mut self, header: &HeaderV4) -> Result<T> {
    let capacity: usize = T::comp_size(header) as usize;
    let position: u64 = self.seek(T::offset(header));

    // Clear the buffer and ensure we have enough capacity
    self.buffer.clear();
    self.buffer.resize(capacity, 0);

    // We only need to operate with a slice of the buffer
    let window: &mut [u8] = &mut self.buffer[..capacity];

    // Seek to the table and read into the buffer
    self.reader.seek_start(position)?;
    self.reader.read_bytes(window)?;

    // Verify table size.
    if window.len() as u64 != T::comp_size(header) {
      return Err(Error::new(ErrorKind::InvalidLen(T::NAME)));
    }

    // Verify MD5
    if DigestMd5::new(window) != T::digest(header) {
      return Err(Error::new(ErrorKind::InvalidMd5(T::NAME)));
    }

    // Read the table header
    let header: T::Header = T::Header::from_slice(window)?;
    let window: &mut [u8] = &mut window[T::Header::SIZE..];

    // Decrypt the extended table
    utils::decrypt(window, T::HKEY)?;

    // Read the extended table
    T::from_slice(header, window)
  }

  fn signature(&mut self, header: &Header) -> Result<Option<Signature>> {
    let data_end: u64 = self.seek(u64::from(header.archive_size));
    let file_end: u64 = self.reader.size();

    // Check if the file has more data after the archive
    if data_end >= file_end {
      return Ok(None);
    }

    // Check if we have enough space for a signature
    if file_end - data_end < Signature::SIZE as u64 {
      return Ok(None);
    }

    self.reader.seek_start(data_end)?;
    self.reader.read_bytes(&mut self.magic)?;

    if self.magic == MAGIC_SIGN {
      Ok(Some(Signature::from_reader(Magic::SIGN, &mut self.reader)?))
    } else {
      Ok(None)
    }
  }
}

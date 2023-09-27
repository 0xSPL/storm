use std::io::Cursor;
use storm_utils::traits::ReadExt;
use storm_utils::traits::SeekExt;

use crate::error::Result;
use crate::extract::FilePtr;
use crate::parse::Handle;
use crate::utils;

// =============================================================================
// Sector Offset Table
// =============================================================================

#[allow(dead_code)]
#[derive(Debug)]
pub struct Sectors {
  pub(crate) count: u32,
  pub(crate) table: Vec<u32>,
}

impl Sectors {
  // TODO: Replace with iterator
  pub(crate) fn new_fake(pointer: &FilePtr<'_>) -> Self {
    let sector_size: u32 = pointer.archive.sector_size();
    let sector_count: u32 = sector_count(pointer);

    let mut offsets: Vec<u32> = (0..sector_count).map(|count| count * sector_size).collect();

    offsets.push(pointer.btentry.comp_size);

    let sectors: Self = Self {
      count: sector_count,
      table: offsets,
    };

    // Sanity Checks
    sectors.assert(pointer);
    sectors
  }

  pub(crate) fn read(pointer: &FilePtr<'_>, enc_key: u32) -> Result<Self> {
    // Sectors are only present if the file is compressed and NOT a single unit
    assert!(!pointer.btentry.is_single_unit());
    assert!(pointer.btentry.is_any_compression());

    // Determine the number of sector offsets
    let sector_count: u32 = sector_count(pointer);
    let offset_count: usize = offset_count(pointer, sector_count);

    // Allocate buffer for sector offset table
    let mut buffer: Vec<u8> = vec![0; offset_count * 4];
    let mut reader: Handle = pointer.archive.handle.duplicate(buffer.len())?;

    // Move the cursor to the table and read into the buffer
    reader.seek_start(pointer.offset())?;
    reader.read_bytes(&mut buffer)?;

    // Decrypt the offset table (if necessary)
    //
    // Note: The sector table is encrypted with `enc_key - 1`
    if pointer.btentry.is_encrypted() {
      utils::decrypt(&mut buffer, enc_key - 1)?;
    }

    // Allocate buffer for table entries
    let mut offsets: Vec<u32> = Vec::with_capacity(offset_count);
    let mut reader: Cursor<Vec<u8>> = Cursor::new(buffer);

    // Read all offsets in the table
    for _ in 0..offset_count {
      offsets.push(reader.read_u32_le()?);
    }

    let sectors: Self = Self {
      count: sector_count,
      table: offsets,
    };

    // Sanity Checks
    sectors.assert(pointer);

    Ok(sectors)
  }

  // TODO: Maybe return errors
  fn assert(&self, pointer: &FilePtr<'_>) {
    // Sector count can't be zero
    assert!(self.count > 0);

    // Sector table can't be empty
    assert!(self.table.len() > 1);

    // Last sector is just an indicator of the compressed size
    assert!(self.table[self.count as usize] == pointer.btentry.comp_size);
  }
}

// Determine the number of data sectors
#[inline]
fn sector_count(pointer: &FilePtr<'_>) -> u32 {
  ((pointer.btentry.file_size - 1) / pointer.archive.sector_size()) + 1
}

// Determine the number of sector offsets
//
// Note: last entry is simply total size
// Note: checksum adds an additional sector
#[inline]
fn offset_count(pointer: &FilePtr<'_>, sector_count: u32) -> usize {
  if pointer.btentry.is_sector_crc() {
    sector_count as usize + 2
  } else {
    sector_count as usize + 1
  }
}

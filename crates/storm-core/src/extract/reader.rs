use storm_utils::traits::ReadExt;
use storm_utils::traits::SeekExt;

use crate::error::Result;
use crate::extract::FilePtr;
use crate::extract::Sectors;
use crate::parse::Handle;
use crate::types::File;
use crate::utils;

pub fn read_file(pointer: FilePtr<'_>) -> Result<File> {
  let enc_key: u32 = pointer.encryption_key();

  // TODO: investigate
  if pointer.btentry.comp_size == 0 || pointer.btentry.file_size == 0 {
    return Ok(File::empty());
  }

  if pointer.btentry.is_patch_file() {
    read_patch(pointer, enc_key)
  } else if pointer.btentry.is_single_unit() {
    read_single_unit(pointer, enc_key)
  } else {
    read_sectors(pointer, enc_key)
  }
}

// =============================================================================
// File Readers
// =============================================================================

fn read_patch(_pointer: FilePtr<'_>, _enc_key: u32) -> Result<File> {
  panic!("read_patch");
}

fn read_single_unit(pointer: FilePtr<'_>, enc_key: u32) -> Result<File> {
  // Allocate buffer for file data
  let mut output: Vec<u8> = vec![0; pointer.btentry.file_size as usize];
  let mut cursor: usize = 0;

  // Allocate buffer for compressed data
  let mut buffer: Vec<u8> = vec![0; pointer.btentry.comp_size as usize];
  let mut reader: Handle = pointer.archive.handle.duplicate_exact()?;

  // Move the reader to the sector offset and read all "compressed" data
  reader.seek_start(pointer.offset())?;
  reader.read_bytes(&mut buffer)?;

  // Decrypt if necessary
  if pointer.btentry.is_encrypted() {
    utils::decrypt(&mut buffer, enc_key)?;
  }

  cursor += read_chunk(&pointer, &buffer, &mut output)?;

  // Sanity Check - We read enough data as indicated by `file_size`
  assert!(cursor as u32 == pointer.btentry.file_size);

  Ok(File::new(output))
}

fn read_sectors(pointer: FilePtr<'_>, enc_key: u32) -> Result<File> {
  if pointer.btentry.is_any_compression() {
    read_sectors_compressed(pointer, enc_key)
  } else {
    read_sectors_uncompressed(pointer, enc_key)
  }
}

fn read_sectors_uncompressed(pointer: FilePtr<'_>, enc_key: u32) -> Result<File> {
  // Create a "fake" sector offset table and use for reading
  read_from_sectors(pointer, enc_key, Sectors::new_fake(&pointer))
}

fn read_sectors_compressed(pointer: FilePtr<'_>, enc_key: u32) -> Result<File> {
  // Parse a real sector offset table and read as applicable
  read_from_sectors(pointer, enc_key, Sectors::read(&pointer, enc_key)?)
}

fn read_from_sectors(pointer: FilePtr<'_>, enc_key: u32, sectors: Sectors) -> Result<File> {
  // Note: Optimization - we may not need to allocate for an entire sector so
  // limit buffer size to expected size of the file.
  let sector_size: u32 = pointer.archive.sector_size();
  let sector_size: u32 = sector_size.min(pointer.btentry.file_size);

  // Allocate buffer for file data
  let mut output: Vec<u8> = vec![0; pointer.btentry.file_size as usize];
  let mut cursor: usize = 0;

  // Allocate buffer for sector data
  let mut buffer: Vec<u8> = vec![0; sector_size as usize];
  let mut reader: Handle = pointer.archive.handle.duplicate_exact()?;

  // Iterate over all sectors in the file
  //
  // TODO: Use array_windows when stable
  // https://doc.rust-lang.org/std/primitive.slice.html#method.array_windows
  for (index, slice) in sectors.table.windows(2).enumerate() {
    let [this, next] = slice else {
      panic!("Invalid Window Length");
    };

    let length: usize = (next - this) as usize;

    // Sanity Check - length should NOT be larger than target sector size
    assert!(length as u32 <= sector_size);

    // We only need need to operate on a small slice of the sector data buffer
    let window: &mut [u8] = &mut buffer[..length];
    let output: &mut [u8] = &mut output[cursor..];

    // Move the reader to the sector offset and read all "compressed" data
    reader.seek_start(pointer.offset() + u64::from(*this))?;
    reader.read_bytes(window)?;

    // Decrypt if necessary
    //
    // Note: Each sector is encrypted using the key + the 0-based index of the
    //       sector in the file.
    if pointer.btentry.is_encrypted() {
      utils::decrypt(window, enc_key + index as u32)?;
    }

    if pointer.btentry.is_sector_crc() {
      panic!("TODO: CRC");
    }

    cursor += read_chunk(&pointer, window, output)?;
  }

  // Sanity Check - We read enough data as indicated by `file_size`
  assert!(cursor as u32 == pointer.btentry.file_size);

  Ok(File::new(output))
}

fn read_chunk(pointer: &FilePtr<'_>, buffer: &[u8], output: &mut [u8]) -> Result<usize> {
  // Check if this data is really compressed
  let fake: bool = fake_compression(pointer, buffer.len(), output.len());

  let bytes: usize = if !fake && pointer.btentry.is_compressed() {
    utils::decompress(buffer, output)?
  } else if !fake && pointer.btentry.is_imploded() {
    utils::decompress_pkware(buffer, output)?
  } else {
    output[..buffer.len()].copy_from_slice(buffer);
    buffer.len()
  };

  Ok(bytes)
}

fn fake_compression(pointer: &FilePtr<'_>, window: usize, output: usize) -> bool {
  // Sector could not be compressed
  if window == pointer.btentry.file_size as usize {
    return true;
  }

  if window == pointer.archive.sector_size() as usize {
    return true;
  }

  if window == output {
    return true;
  }

  false
}

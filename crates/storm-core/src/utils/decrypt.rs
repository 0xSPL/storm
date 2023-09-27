include!(concat!(env!("OUT_DIR"), "/cryptable.rs"));

use byteorder::ByteOrder;
use byteorder::LE;

use crate::error::Result;

const SEED: u32 = 0xEEEEEEEE;

/// Decrypt `buffer` with the given encryption `key`.
pub fn decrypt(buffer: &mut [u8], mut key: u32) -> Result<()> {
  let length: usize = buffer.len() >> 0x2;
  let mut seed: u32 = SEED;

  for index in 0..length {
    seed = seed.wrapping_add(CRYPTABLE[0x400 + (key & 0xFF) as usize]);

    let data: &mut [u8] = &mut buffer[index << 0x2..];
    let word: u32 = LE::read_u32(data);
    let word: u32 = word ^ key.wrapping_add(seed);

    LE::write_u32(data, word);

    key = (!key << 0x15).wrapping_add(0x11111111) | (key >> 0x0B);

    seed = word
      .wrapping_add(seed)
      .wrapping_add(seed << 0x5)
      .wrapping_add(0x3);
  }

  Ok(())
}

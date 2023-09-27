include!(concat!(env!("OUT_DIR"), "/ascii_to_upper_sensitive.rs"));
include!(concat!(env!("OUT_DIR"), "/cryptable.rs"));

// =============================================================================
// Hash Type
// =============================================================================

/// Available hash types.
#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum HashType {
  Table = 0x000,
  NameA = 0x100,
  NameB = 0x200,
  File = 0x300,
}

// =============================================================================
// Hash Function
// =============================================================================

const SEED1: u32 = 0x7FED7FED;
const SEED2: u32 = 0xEEEEEEEE;

/// Hash `name` with the given `hash` type.
pub fn hash(name: &str, hash: HashType) -> u32 {
  let mut seed1: u32 = SEED1;
  let mut seed2: u32 = SEED2;

  for byte in name.bytes() {
    let ch: u32 = u32::from(ASCII_TO_UPPER_SENSITIVE[usize::from(byte)]);

    seed1 = CRYPTABLE[(hash as u32 + ch) as usize] ^ seed1.wrapping_add(seed2);

    seed2 = ch
      .wrapping_add(seed1)
      .wrapping_add(seed2)
      .wrapping_add(seed2 << 0x5)
      .wrapping_add(0x3);
  }

  seed1
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::consts;

  #[test]
  fn test_hash() {
    assert_eq!(hash("(hash table)", HashType::File), consts::HASH_KEY_HT);
    assert_eq!(hash("(block table)", HashType::File), consts::HASH_KEY_BT);
    assert_eq!(hash("(listfile)", HashType::Table), consts::HASH_KEY_LF);
  }
}

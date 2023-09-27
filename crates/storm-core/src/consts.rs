// =============================================================================
// Parsing
// =============================================================================

// Malformed MPQs may contain crazy block positions - we need to "fix" them
pub const BT_MASK: u32 = 0x0FFFFFFF;

// =============================================================================
// Signatures
// =============================================================================

/// Signature for [MPQ Header][crate::types::Header].
pub const MAGIC_ID: [u8; 4] = *b"MPQ\x1A";

/// Signature for [MPQ User Data][crate::types::UserData].
pub const MAGIC_UD: [u8; 4] = *b"MPQ\x1B";

/// Signature for [HET Table][crate::types::ExtHTable].
pub const MAGIC_HET: [u8; 4] = *b"HET\x1A";

/// Signature for [BET Table][crate::types::ExtBTable].
pub const MAGIC_BET: [u8; 4] = *b"BET\x1A";

/// Signature for [`strong digital signature`][crate::types::Signature].
pub const MAGIC_SIGN: [u8; 4] = *b"NGIS";

// =============================================================================
// Header Versions
// =============================================================================

/// Original Format.
pub const V1: u16 = 0x0000;

/// Burning Crusade.
pub const V2: u16 = 0x0001;

/// WoW Cataclysm Beta.
pub const V3: u16 = 0x0002;

/// WoW Cataclysm.
pub const V4: u16 = 0x0003;

// =============================================================================
// Hashing
// =============================================================================

/// Hash Key: `hash("(hash table)", HashType::File)`.
pub const HASH_KEY_HT: u32 = 0xC3AF3770;

/// Hash Key: `hash("(block table)", HashType::File)`.
pub const HASH_KEY_BT: u32 = 0xEC83B3A3;

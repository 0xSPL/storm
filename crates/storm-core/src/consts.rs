// =============================================================================
// Parsing
// =============================================================================

// Malformed MPQs may contain crazy block positions - we need to "fix" them
pub const BT_MASK: u32 = 0x0FFFFFFF;

// =============================================================================
// Signatures
// =============================================================================

pub const MAGIC_ID: [u8; 4] = *b"MPQ\x1A";

pub const MAGIC_UD: [u8; 4] = *b"MPQ\x1B";

pub const MAGIC_HET: [u8; 4] = *b"HET\x1A";

pub const MAGIC_BET: [u8; 4] = *b"BET\x1A";

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

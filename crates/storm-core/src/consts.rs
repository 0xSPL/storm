// =============================================================================
// Parsing
// =============================================================================

/// Size of disk sector boundary.
pub const HJUMP: u64 = 0x200;

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

/// Hash Key: `hash("(listfile)", HashType::Table)`.
pub const HASH_KEY_LF: u32 = 0x5F3DE859;

// =============================================================================
// Compression
// =============================================================================

/// Compression: Huffman Encoded.
pub const COMP_HUFFMAN: u8 = 0x01;

/// Compression: ZLib.
pub const COMP_ZLIB: u8 = 0x02;

/// Compression: PKWare Data Compression Library.
pub const COMP_PKWARE: u8 = 0x08;

/// Compression: BZip2.
pub const COMP_BZIP2: u8 = 0x10;

/// Compression: Sparse.
pub const COMP_SPARSE: u8 = 0x20;

/// Compression: IMA ADPCM (mono).
pub const COMP_IMA_ADPCM_1C: u8 = 0x40;

/// Compression: IMA ADPCM (stereo).
pub const COMP_IMA_ADPCM_2C: u8 = 0x80;

/// Compression: LZMA.
pub const COMP_LMZA: u8 = 0x12;

/// Compression: Sparse + ZLib.
pub const COMP_SPARSE_ZLIB: u8 = COMP_SPARSE | COMP_ZLIB;

/// Compression: Sparse + BZip2.
pub const COMP_SPARSE_BZIP2: u8 = COMP_SPARSE | COMP_BZIP2;

/// Compression: IMA ADPCM (mono) + PKWare.
pub const COMP_IMA_ADPCM_1C_PKWARE: u8 = COMP_IMA_ADPCM_1C | COMP_PKWARE;

/// Compression: IMA ADPCM (stereo) + PKWare.
pub const COMP_IMA_ADPCM_2C_PKWARE: u8 = COMP_IMA_ADPCM_2C | COMP_PKWARE;

/// Compression: IMA ADPCM (mono) + Huffman.
pub const COMP_IMA_ADPCM_1C_HUFFMAN: u8 = COMP_IMA_ADPCM_1C | COMP_HUFFMAN;

/// Compression: IMA ADPCM (stereo) + Huffman.
pub const COMP_IMA_ADPCM_2C_HUFFMAN: u8 = COMP_IMA_ADPCM_2C | COMP_HUFFMAN;

use core::mem::size_of;
use storm_utils::traits::ParseContext;
use storm_utils::traits::ReadExt;
use storm_utils::utils::Digest;

use crate::error::Error;
use crate::types::Magic;

// =============================================================================
// Static Assertions
// =============================================================================

const_assert_size!(Signature, 0x104);

// =============================================================================
// Strong Digital Signature
// =============================================================================

/// Strong Digital Signature.
///
/// ## Layout
///
/// `0x00` = `magic` \
/// `0x04` = `signature`
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Signature {
  /// Indicates the presence of a digital signature.
  pub magic: Magic,
  /// The digital signature.
  pub bytes: Digest<0x100>,
}

impl Signature {
  /// The size of a signature block.
  pub const SIZE: usize = size_of::<Self>();
}

impl ParseContext<Magic> for Signature {
  type Error = Error;

  /// Parse a strong digital signature from the given `reader`.
  fn from_reader<R: ReadExt + ?Sized>(context: Magic, reader: &mut R) -> Result<Self, Self::Error> {
    debug_assert_eq!(context, Magic::SIGN);

    Ok(Self {
      magic: context,
      bytes: reader.read_array_u8()?.into(),
    })
  }
}

only_serde! {
  use serde::ser::SerializeStruct;
  use serde::Serialize;
  use serde::Serializer;

  impl Serialize for Signature {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
      let mut state: S::SerializeStruct = serializer.serialize_struct("Signature", 2)?;
      state.serialize_field("magic", &self.magic)?;
      state.serialize_field("bytes", &self.bytes)?;
      state.end()
    }
  }
}

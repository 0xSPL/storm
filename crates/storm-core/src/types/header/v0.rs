use core::ops::Deref;
use storm_utils::traits::ParseContext;
use storm_utils::traits::ReadExt;

use crate::consts;
use crate::error::Error;
use crate::types::HeaderV1;
use crate::types::HeaderV2;
use crate::types::HeaderV3;
use crate::types::HeaderV4;
use crate::types::Magic;

// =============================================================================
// Header
// =============================================================================

/// Archive Header.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Header {
  V1(HeaderV1),
  V2(HeaderV2),
  V3(HeaderV3),
  V4(HeaderV4),
}

impl Header {
  pub const VER1: u16 = consts::V1;
  pub const VER2: u16 = consts::V2;
  pub const VER3: u16 = consts::V3;
  pub const VER4: u16 = consts::V4;

  /// Returns the internal size of the header.
  #[inline]
  pub const fn size(&self) -> usize {
    match self {
      Self::V1(_) => HeaderV1::SIZE,
      Self::V2(_) => HeaderV2::SIZE,
      Self::V3(_) => HeaderV3::SIZE,
      Self::V4(_) => HeaderV4::SIZE,
    }
  }

  /// Returns `true` if this is a V1 header.
  #[inline]
  pub const fn is_v1(&self) -> bool {
    matches!(self, Self::V1(_))
  }

  /// Returns `true` if this is a V2 header.
  #[inline]
  pub const fn is_v2(&self) -> bool {
    matches!(self, Self::V2(_))
  }

  /// Returns `true` if this is a V3 header.
  #[inline]
  pub const fn is_v3(&self) -> bool {
    matches!(self, Self::V3(_))
  }

  /// Returns `true` if this is a V4 header.
  #[inline]
  pub const fn is_v4(&self) -> bool {
    matches!(self, Self::V4(_))
  }

  /// Returns a reference to the the V1 header.
  #[inline]
  pub const fn v1(&self) -> &HeaderV1 {
    match self {
      Self::V1(header) => header,
      Self::V2(header) => &header.v1,
      Self::V3(header) => &header.v2.v1,
      Self::V4(header) => &header.v3.v2.v1,
    }
  }

  /// Returns a reference to the the V2 header.
  #[inline]
  pub const fn v2(&self) -> Option<&HeaderV2> {
    match self {
      Self::V1(_) => None,
      Self::V2(header) => Some(header),
      Self::V3(header) => Some(&header.v2),
      Self::V4(header) => Some(&header.v3.v2),
    }
  }

  /// Returns a reference to the the V3 header.
  #[inline]
  pub const fn v3(&self) -> Option<&HeaderV3> {
    match self {
      Self::V1(_) => None,
      Self::V2(_) => None,
      Self::V3(header) => Some(header),
      Self::V4(header) => Some(&header.v3),
    }
  }

  /// Returns a reference to the the V4 header.
  #[inline]
  pub const fn v4(&self) -> Option<&HeaderV4> {
    match self {
      Self::V1(_) => None,
      Self::V2(_) => None,
      Self::V3(_) => None,
      Self::V4(header) => Some(header),
    }
  }
}

impl Deref for Header {
  type Target = HeaderV1;

  #[inline]
  fn deref(&self) -> &Self::Target {
    self.v1()
  }
}

impl ParseContext<Magic> for Header {
  type Error = Error;

  /// Parse header from the given `reader`.
  fn from_reader<R: ReadExt + ?Sized>(context: Magic, reader: &mut R) -> Result<Self, Self::Error> {
    let v1: HeaderV1 = reader.parse_context(context)?;

    // Parse more fields, depending on the version
    match v1.format_version {
      Self::VER1 => Ok(Self::V1(v1)),
      Self::VER2 => reader.parse_context(v1).map(Self::V2),
      Self::VER3 => reader.parse_context(v1).map(Self::V3),
      Self::VER4 => reader.parse_context(v1).map(Self::V4),
      _ => panic!("Handle Unknown Version"),
    }
  }
}

only_serde! {
  use serde::Serialize;
  use serde::Serializer;

  impl Serialize for Header {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
      match self {
        Self::V1(header) => header.serialize(serializer),
        Self::V2(header) => header.serialize(serializer),
        Self::V3(header) => header.serialize(serializer),
        Self::V4(header) => header.serialize(serializer),
      }
    }
  }
}

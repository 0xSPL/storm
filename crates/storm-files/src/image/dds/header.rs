use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use storm_core::error::Error;
use storm_utils::traits::Parse;
use storm_utils::traits::ReadExt;

use crate::image::dds::ColorType;
use crate::image::dds::PixelFormat;

// =============================================================================
// Header
//
// https://learn.microsoft.com/en-us/windows/win32/direct3ddds/dds-header
// =============================================================================

/// Describes a DDS file header.
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Header {
  /// Size of structure.
  pub size: u32,
  /// Flags to indicate which members contain valid data.
  pub flags: HeaderFlags,
  /// Surface height (in pixels).
  pub height: u32,
  /// Surface width (in pixels).
  pub width: u32,
  /// The pitch or number of bytes per scan line in an uncompressed
  /// texture; the total number of bytes in the top level texture for
  /// a compressed texture.
  pub pitch: u32,
  /// Depth of a volume texture (in pixels).
  pub depth: u32,
  /// Number of mipmap levels.
  pub mipmap_count: u32,
  /// Unused.
  pub reserved1: [u8; 11 * 4],
  /// The pixel format.
  pub format: PixelFormat,
  /// Specifies the complexity of the surfaces stored.
  pub caps1: Caps1,
  /// Additional detail about the surfaces stored.
  pub caps2: Caps2,
  /// Unused.
  pub caps3: u32,
  /// Unused.
  pub caps4: u32,
  /// Unused.
  pub reserved2: u32,
}

impl Header {
  #[inline]
  pub const fn color_type(&self) -> ColorType {
    self.format.color_type()
  }

  pub fn layers(&self) -> Vec<(u32, u32)> {
    (0..self.mipmap_count.max(1))
      .map(|index| self.layer_size(index))
      .collect()
  }

  #[inline]
  pub fn layer_size(&self, layer: u32) -> (u32, u32) {
    (
      (self.width / 2u32.pow(layer)).max(1),
      (self.height / 2u32.pow(layer)).max(1),
    )
  }
}

impl Debug for Header {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("Header")
      .field("size", &self.size)
      .field("flags", &self.flags)
      .field("height", &self.height)
      .field("width", &self.width)
      .field("pitch", &self.pitch)
      .field("depth", &self.depth)
      .field("mipmap_count", &self.mipmap_count)
      .field("format", &self.format)
      .field("caps1", &self.caps1)
      .field("caps2", &self.caps2)
      .finish()
  }
}

impl Parse for Header {
  type Error = Error;

  fn from_reader<R: ReadExt + ?Sized>(reader: &mut R) -> Result<Self, Self::Error> {
    Ok(Self {
      size: reader.read_u32_le()?,
      flags: HeaderFlags::from_value(reader.read_u32_le()?),
      height: reader.read_u32_le()?,
      width: reader.read_u32_le()?,
      pitch: reader.read_u32_le()?,
      depth: reader.read_u32_le()?,
      mipmap_count: reader.read_u32_le()?,
      reserved1: reader.read_array_u8()?,
      format: reader.parse()?,
      caps1: Caps1::from_value(reader.read_u32_le()?),
      caps2: Caps2::from_value(reader.read_u32_le()?),
      caps3: reader.read_u32_le()?,
      caps4: reader.read_u32_le()?,
      reserved2: reader.read_u32_le()?,
    })
  }
}

// =============================================================================
// Header Flags
// =============================================================================

bitflags! {
  #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
  pub struct HeaderFlags: u32 {
    /// Required in every .dds file.
    const CAPS = 0x00000001;
    /// Required in every .dds file.
    const HEIGHT = 0x00000002;
    /// Required in every .dds file.
    const WIDTH = 0x00000004;
    /// Required when pitch is provided for an uncompressed texture.
    const PITCH = 0x00000008;
    /// Required in every .dds file.
    const PIXELFORMAT = 0x00001000;
    /// Required in a mipmapped texture.
    const MIPMAPCOUNT = 0x00020000;
    /// Required when pitch is provided for a compressed texture.
    const LINEARSIZE = 0x00080000;
    /// Required in a depth texture.
    const DEPTH = 0x00800000;
  }
}

// =============================================================================
// Caps1 Flags
// =============================================================================

bitflags! {
  #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
  pub struct Caps1: u32 {
    /// Optional; must be used on any file that contains more than one surface
    /// (a mipmap, a cubic environment map, or mipmapped volume texture).
    const COMPLEX = 0x00000008;
    /// Optional; should be used for a mipmap.
    const MIPMAP = 0x00400000;
    /// Required.
    const TEXTURE = 0x00001000;
  }
}

// =============================================================================
// Caps2 Flags
// =============================================================================

bitflags! {
  #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
  pub struct Caps2: u32 {
    /// Required for a cube map.
    const CUBEMAP = 0x00000200;
    /// Required when these surfaces are stored in a cube map.
    const CUBEMAP_POSITIVEX = 0x00000400;
    /// Required when these surfaces are stored in a cube map.
    const CUBEMAP_NEGATIVEX = 0x00000800;
    /// Required when these surfaces are stored in a cube map.
    const CUBEMAP_POSITIVEY = 0x00001000;
    /// Required when these surfaces are stored in a cube map.
    const CUBEMAP_NEGATIVEY = 0x00002000;
    /// Required when these surfaces are stored in a cube map.
    const CUBEMAP_POSITIVEZ = 0x00004000;
    /// Required when these surfaces are stored in a cube map.
    const CUBEMAP_NEGATIVEZ = 0x00008000;
    /// Required for a volume texture.
    const VOLUME = 0x00200000;
  }
}

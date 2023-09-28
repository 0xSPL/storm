use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use storm_core::error::Error;
use storm_utils::traits::Parse;
use storm_utils::traits::ReadExt;

use crate::image::dds::ColorType;

// =============================================================================
// Pixel Format
//
// https://learn.microsoft.com/en-us/windows/win32/direct3ddds/dds-pixelformat
// =============================================================================

/// Surface pixel format.
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct PixelFormat {
  /// Structure size.
  pub size: u32,
  /// Values which indicate what type of data is in the surface.
  pub flags: PixelFormatFlags,
  /// Four-character codes for specifying compressed or custom formats.
  pub fourcc: [u8; 4],
  /// Number of bits in an RGB (possibly including alpha) format.
  pub rgb_bit_count: u32,
  /// Red (or luminance or Y) mask for reading color data.
  pub rbit_mask: u32,
  /// Green (or U) mask for reading color data.
  pub gbit_mask: u32,
  /// Blue (or V) mask for reading color data.
  pub bbit_mask: u32,
  /// Alpha mask for reading alpha data.
  pub abit_mask: u32,
}

impl PixelFormat {
  pub const NONE: [u8; 4] = [0, 0, 0, 0];
  pub const DXT1: [u8; 4] = *b"DXT1";
  pub const DXT2: [u8; 4] = *b"DXT2";
  pub const DXT3: [u8; 4] = *b"DXT3";
  pub const DXT4: [u8; 4] = *b"DXT4";
  pub const DXT5: [u8; 4] = *b"DXT5";
  pub const DX10: [u8; 4] = *b"DX10";

  #[inline]
  pub const fn color_type(&self) -> ColorType {
    match (
      self.rgb_bit_count,
      self.rbit_mask,
      self.gbit_mask,
      self.bbit_mask,
      self.abit_mask,
    ) {
      (32, 0x00FF0000, 0x0000FF00, 0x000000FF, 0xFF000000) => ColorType::A8R8G8B8,
      _ => panic!("PixelFormat::color_type"),
    }
  }

  #[inline]
  pub const fn fourcc_str(&self) -> &str {
    if !self.flags.contains(PixelFormatFlags::FOURCC) {
      return "NONE";
    }

    match self.fourcc {
      Self::DXT1 => "DXT1",
      Self::DXT2 => "DXT2",
      Self::DXT3 => "DXT3",
      Self::DXT4 => "DXT4",
      Self::DXT5 => "DXT5",
      Self::DX10 => "DX10",
      _ => "UNKNOWN",
    }
  }

  #[inline]
  pub(crate) const fn mask_r(&self, pixel: u32) -> u8 {
    Self::mask(pixel, self.rbit_mask)
  }

  #[inline]
  pub(crate) const fn mask_g(&self, pixel: u32) -> u8 {
    Self::mask(pixel, self.gbit_mask)
  }

  #[inline]
  pub(crate) const fn mask_b(&self, pixel: u32) -> u8 {
    Self::mask(pixel, self.bbit_mask)
  }

  #[inline]
  pub(crate) const fn mask_a(&self, pixel: u32) -> u8 {
    Self::mask(pixel, self.abit_mask)
  }

  #[inline]
  const fn mask(pixel: u32, mask: u32) -> u8 {
    ((pixel & mask) >> (mask.trailing_zeros() * 255 / (2u32.pow(mask.count_ones()) - 1))) as u8
  }
}

impl Debug for PixelFormat {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("PixelFormat")
      .field("size", &self.size)
      .field("flags", &self.flags)
      .field("fourcc", &self.fourcc_str())
      .field("rgb_bit_count", &self.rgb_bit_count)
      .field("rbit_mask", &format_args!("{:#010X}", self.rbit_mask))
      .field("gbit_mask", &format_args!("{:#010X}", self.gbit_mask))
      .field("bbit_mask", &format_args!("{:#010X}", self.bbit_mask))
      .field("abit_mask", &format_args!("{:#010X}", self.abit_mask))
      .finish()
  }
}

impl Parse for PixelFormat {
  type Error = Error;

  fn from_reader<R: ReadExt + ?Sized>(reader: &mut R) -> Result<Self, Self::Error> {
    Ok(Self {
      size: reader.read_u32_le()?,
      flags: PixelFormatFlags::from_value(reader.read_u32_le()?),
      fourcc: reader.read_array_u8()?,
      rgb_bit_count: reader.read_u32_le()?,
      rbit_mask: reader.read_u32_le()?,
      gbit_mask: reader.read_u32_le()?,
      bbit_mask: reader.read_u32_le()?,
      abit_mask: reader.read_u32_le()?,
    })
  }
}

// =============================================================================
// Pixel Format Flags
// =============================================================================

bitflags! {
  #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
  pub struct PixelFormatFlags: u32 {
    /// Texture contains alpha data;
    const ALPHAPIXELS = 0x00000001;
    /// Used in some older DDS files for alpha channel only uncompressed data.
    const ALPHA = 0x00000002;
    /// Texture contains compressed RGB data.
    const FOURCC = 0x00000004;
    /// Texture contains uncompressed RGB data.
    const RGB = 0x00000040;
    /// Used in some older DDS files for YUV uncompressed data.
    const YUV = 0x00000200;
    /// Used in some older DDS files for single channel color uncompressed data.
    const LUMINANCE = 0x00020000;
  }
}

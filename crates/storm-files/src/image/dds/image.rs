#![allow(deprecated)]

use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;
use image::codecs::dxt::DxtDecoder;
use image::codecs::dxt::DxtVariant;
use image::DynamicImage;
use image::ImageFormat;
use image::RgbaImage;
use std::path::Path;
use storm_core::error::Error;
use storm_core::error::ErrorKind;
use storm_core::error::Result;
use storm_utils::traits::Parse;
use storm_utils::traits::ParseContext;
use storm_utils::traits::ReadExt;

use crate::image::dds::ColorType;
use crate::image::dds::Header;
use crate::image::dds::HeaderFlags;
use crate::image::dds::PixelFormat;
use crate::image::dds::PixelFormatFlags;

// =============================================================================
// DDS Image
//
// https://learn.microsoft.com/en-us/windows/win32/direct3ddds/dx-graphics-dds-pguide
// =============================================================================

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Image {
  magic: [u8; 4],
  header: Header,
  layers: Layers,
}

impl Image {
  pub const MAGIC: [u8; 4] = *b"DDS ";

  #[inline]
  pub const fn magic(&self) -> [u8; 4] {
    self.magic
  }

  #[inline]
  pub const fn header(&self) -> &Header {
    &self.header
  }

  #[inline]
  pub const fn layers(&self) -> &Layers {
    &self.layers
  }

  pub fn export_png<P>(&self, path: &P, index: usize) -> Result<()>
  where
    P: AsRef<Path> + ?Sized,
  {
    if let Some(image) = self.layers.get(index) {
      image
        .save_with_format(path.as_ref().with_extension("png"), ImageFormat::Png)
        .map_err(|error| Error::new_std(ErrorKind::Other, error))?;
    }

    Ok(())
  }
}

impl Parse for Image {
  type Error = Error;

  fn from_reader<R: ReadExt + ?Sized>(reader: &mut R) -> Result<Self, Self::Error> {
    let magic: [u8; 4] = reader.read_array_u8()?;

    assert!(magic == Self::MAGIC);

    let header: Header = reader.parse()?;

    assert!(header.size == 124); // Must be 126
    assert!(header.flags.contains(HeaderFlags::CAPS));
    assert!(header.flags.contains(HeaderFlags::HEIGHT));
    assert!(header.flags.contains(HeaderFlags::WIDTH));
    assert!(header.flags.contains(HeaderFlags::PIXELFORMAT));
    assert!(header.format.size == 32); // Must be 32

    let layers: Layers = reader.parse_context(&header)?;

    Ok(Self {
      magic,
      header,
      layers,
    })
  }
}

// =============================================================================
// Layers
// =============================================================================

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Layers {
  inner: Vec<RgbaImage>,
}

impl Layers {
  #[inline]
  pub fn get(&self, index: usize) -> Option<&RgbaImage> {
    self.inner.get(index)
  }

  #[inline]
  pub fn raw(&self, index: usize) -> Option<&[u8]> {
    self.get(index).map(|layer| &**layer.as_raw())
  }
}

impl Debug for Layers {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.write_str("Layers ")?;
    f.debug_list().entries(self.iter()).finish()
  }
}

impl Deref for Layers {
  type Target = [RgbaImage];

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

impl ParseContext<&'_ Header> for Layers {
  type Error = Error;

  fn from_reader<R: ReadExt + ?Sized>(
    context: &Header,
    reader: &mut R,
  ) -> Result<Self, Self::Error> {
    if context.format.flags.contains(PixelFormatFlags::FOURCC) {
      let variant: DxtVariant = match context.format.fourcc {
        PixelFormat::DXT1 => DxtVariant::DXT1,
        PixelFormat::DXT2 => panic!("Handle PixelFormat::DXT2"),
        PixelFormat::DXT3 => DxtVariant::DXT3,
        PixelFormat::DXT4 => panic!("Handle PixelFormat::DXT4"),
        PixelFormat::DXT5 => DxtVariant::DXT5,
        PixelFormat::DX10 => panic!("Handle PixelFormat::DX10"),
        _ => panic!("Handle {}", context.format.fourcc_str()),
      };

      DxtDecoder::new(reader, context.width, context.height, variant)
        .and_then(DynamicImage::from_decoder)
        .map_err(|error| Error::new_std(ErrorKind::Other, error))
        .map(|dynamic| dynamic.into_rgba8())
        .map(|image| Self { inner: vec![image] })
    } else if context.format.flags.contains(PixelFormatFlags::RGB) {
      if context.format.flags.contains(PixelFormatFlags::ALPHA) {
        panic!("Handle PixelFormatFlags::ALPHA");
      }

      if context.format.flags.contains(PixelFormatFlags::YUV) {
        panic!("Handle PixelFormatFlags::YUV");
      }

      if context.format.flags.contains(PixelFormatFlags::LUMINANCE) {
        panic!("Handle PixelFormatFlags::LUMINANCE");
      }

      let kind: ColorType = context.color_type();
      let size: u32 = context.width * context.height * kind.bytes_per_pixel() as u32;
      let data: Vec<u8> = reader.read_all(size as usize)?;

      Ok(Self {
        inner: decode_uncompressed(context, &data)?,
      })
    } else {
      panic!("Fail Unknown");
    }
  }
}

// =============================================================================
// Private
// =============================================================================

fn decode_uncompressed(header: &Header, mut buffer: &[u8]) -> Result<Vec<RgbaImage>> {
  let kind: ColorType = header.color_type();
  let maps: Vec<(u32, u32)> = header.layers();
  let bpp: u32 = kind.bytes_per_pixel() as u32;

  let mut layers: Vec<RgbaImage> = Vec::with_capacity(maps.len());

  for (w, h) in maps {
    let bytes: usize = (w * h * bpp) as usize;
    let slice: &[u8] = &buffer[..bytes];

    // Sanity Check - slice is correct length
    assert!(slice.len() % bpp as usize == 0);

    let mut layer: Vec<u8> = Vec::with_capacity(bytes);

    for chunk in slice.chunks_exact(bpp as usize) {
      assert!(chunk.len() == 4); // TODO: Support RGB

      let pixel: u32 = compose_pixel(chunk);

      layer.push(header.format.mask_r(pixel));
      layer.push(header.format.mask_g(pixel));
      layer.push(header.format.mask_b(pixel));
      layer.push(header.format.mask_a(pixel));
    }

    buffer = &buffer[bytes..];
    layers.push(RgbaImage::from_raw(w, h, layer).unwrap());
  }

  // Sanity Check - we read everything in the buffer
  assert!(buffer.is_empty());

  Ok(layers)
}

#[inline]
const fn compose_pixel(chunk: &[u8]) -> u32 {
  let mut rgba: u32 = 0;
  rgba += chunk[0] as u32;
  rgba += (chunk[1] as u32) << 8;
  rgba += (chunk[2] as u32) << 16;
  rgba += (chunk[3] as u32) << 24;
  rgba
}

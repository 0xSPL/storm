use crate::consts;
use crate::error::Error;
use crate::error::ErrorKind;
use crate::error::Result;

#[rustfmt::skip]
pub fn decompress(buffer: &[u8], output: &mut [u8]) -> Result<usize> {
  match buffer {
    [consts::COMP_HUFFMAN, data @ ..] => {
      decompress_huffman(data, output)
    }
    [consts::COMP_ZLIB, data @ ..] => {
      decompress_zlib(data, output)
    }
    [consts::COMP_PKWARE, data @ ..] => {
      decompress_pkware(data, output)
    }
    [consts::COMP_BZIP2, data @ ..] => {
      decompress_bzip2(data, output)
    }
    [consts::COMP_SPARSE, data @ ..] => {
      decompress_sparse(data, output)
    }
    [consts::COMP_IMA_ADPCM_1C, data @ ..] => {
      decompress_adpcm(data, output, 1)
    }
    [consts::COMP_IMA_ADPCM_2C, data @ ..] => {
      decompress_adpcm(data, output, 2)
    }
    [consts::COMP_LMZA, ..] => {
      panic!("TODO: COMP_LMZA")
    }
    [consts::COMP_SPARSE_ZLIB, ..] => {
      panic!("TODO: COMP_SPARSE_ZLIB")
    }
    [consts::COMP_SPARSE_BZIP2, ..] => {
      panic!("TODO: COMP_SPARSE_BZIP2")
    }
    [consts::COMP_IMA_ADPCM_1C_PKWARE, ..] => {
      panic!("TODO: COMP_IMA_ADPCM_1C_PKWARE")
    }
    [consts::COMP_IMA_ADPCM_2C_PKWARE, ..] => {
      panic!("TODO: COMP_IMA_ADPCM_2C_PKWARE")
    }
    [consts::COMP_IMA_ADPCM_1C_HUFFMAN, data @ ..] => {
      let size: usize = decompress_huffman(data, output)?;
      let temp: Vec<u8> = output[..size].to_vec();

      decompress_adpcm(&temp, output, 1)
    }
    [consts::COMP_IMA_ADPCM_2C_HUFFMAN, data @ ..] => {
      let size: usize = decompress_huffman(data, output)?;
      let temp: Vec<u8> = output[..size].to_vec();

      decompress_adpcm(&temp, output, 2)
    }
    [mode] | [mode, ..] => {
      Err(Error::new(ErrorKind::DecompressionInvalid(*mode)))
    }
    [] => {
      Err(Error::new(ErrorKind::DecompressionNoBytes))
    }
  }
}

// =============================================================================
// Compression Format
// =============================================================================

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompressionFormat {
  Adpcm(bool),
  BZip2,
  Deflate,
  Huffman,
  PkWare,
  Sparse,
}

impl CompressionFormat {
  #[inline]
  pub const fn feature(&self) -> &'static str {
    match self {
      Self::Adpcm(_) => "adpcm",
      Self::BZip2 => "bzip2",
      Self::Deflate => "zlib",
      Self::Huffman => "huffman",
      Self::PkWare => "pkware",
      Self::Sparse => "sparse",
    }
  }

  #[inline]
  pub const fn name(&self) -> &'static str {
    match self {
      Self::Adpcm(true) => "IMA ADPCM (stereo) Compression",
      Self::Adpcm(false) => "IMA ADPCM (mono) Compression",
      Self::BZip2 => "BZip2 Compression",
      Self::Deflate => "ZLib Compression",
      Self::Huffman => "Huffman Coding",
      Self::PkWare => "PkWare Compression",
      Self::Sparse => "Sparse Compression",
    }
  }
}

// =============================================================================
// Huffman
// =============================================================================

#[cfg(feature = "huffman")]
pub fn decompress_huffman(buffer: &[u8], output: &mut [u8]) -> Result<usize> {
  let mut stream: storm_huffman::Decompress = storm_huffman::Decompress::new();

  stream
    .decompress(buffer, output)
    .map_err(|error| Error::new_std(ErrorKind::DecompressionFailure, error))?;

  Ok(stream.total_out())
}

#[cfg(not(feature = "huffman"))]
pub fn decompress_huffman(_buffer: &[u8], _output: &mut [u8]) -> Result<usize> {
  Err(Error::new(ErrorKind::DecompressionFeature(
    CompressionFormat::Huffman,
  )))
}

// =============================================================================
// ZLib
// =============================================================================

#[cfg(feature = "zlib")]
pub fn decompress_zlib(buffer: &[u8], output: &mut [u8]) -> Result<usize> {
  const fn translate(status: flate2::Status) -> &'static str {
    match status {
      flate2::Status::Ok => "More output to be written but the output buffer is full.",
      flate2::Status::BufError => "The output buffer isn’t large enough to contain the result.",
      flate2::Status::StreamEnd => unreachable!(),
    }
  }

  let mut stream: flate2::Decompress = flate2::Decompress::new(true);
  let mode: flate2::FlushDecompress = flate2::FlushDecompress::Finish;

  let status: flate2::Status = stream
    .decompress(buffer, output, mode)
    .map_err(|error| Error::new_std(ErrorKind::DecompressionFailure, error))?;

  if let flate2::Status::StreamEnd = status {
    Ok(stream.total_out() as usize)
  } else {
    Err(Error::new(ErrorKind::DecompressionStatus(translate(
      status,
    ))))
  }
}

#[cfg(not(feature = "zlib"))]
pub fn decompress_zlib(_buffer: &[u8], _output: &mut [u8]) -> Result<usize> {
  Err(Error::new(ErrorKind::DecompressionFeature(
    CompressionFormat::Deflate,
  )))
}

// =============================================================================
// PKWare
// =============================================================================

#[cfg(feature = "pkware")]
pub fn decompress_pkware(buffer: &[u8], output: &mut [u8]) -> Result<usize> {
  let mut stream: storm_pklib::Decompress = storm_pklib::Decompress::new();

  stream
    .decompress(buffer, output)
    .map_err(|error| Error::new_std(ErrorKind::DecompressionFailure, error))?;

  Ok(stream.total_out())
}

#[cfg(not(feature = "pkware"))]
pub fn decompress_pkware(_buffer: &[u8], _output: &mut [u8]) -> Result<usize> {
  Err(Error::new(ErrorKind::DecompressionFeature(
    CompressionFormat::PkWare,
  )))
}

// =============================================================================
// BZip2
// =============================================================================

#[cfg(feature = "bzip2")]
pub fn decompress_bzip2(buffer: &[u8], output: &mut [u8]) -> Result<usize> {
  const fn translate(status: bzip2::Status) -> &'static str {
    match status {
      bzip2::Status::Ok => unreachable!(),
      bzip2::Status::FlushOk => "The flush action went ok.",
      bzip2::Status::RunOk => "The run action went ok.",
      bzip2::Status::FinishOk => "The finish action went ok.",
      bzip2::Status::StreamEnd => unreachable!(),
      bzip2::Status::MemNeeded => "Insufficient memory in the input or output buffer.",
    }
  }

  let mut stream: bzip2::Decompress = bzip2::Decompress::new(false);

  let status: bzip2::Status = stream
    .decompress(buffer, output)
    .map_err(|error| Error::new_std(ErrorKind::DecompressionFailure, error))?;

  if let bzip2::Status::Ok | bzip2::Status::StreamEnd = status {
    Ok(stream.total_out() as usize)
  } else {
    Err(Error::new(ErrorKind::DecompressionStatus(translate(
      status,
    ))))
  }
}

#[cfg(not(feature = "bzip2"))]
pub fn decompress_bzip2(_buffer: &[u8], _output: &mut [u8]) -> Result<usize> {
  Err(Error::new(ErrorKind::DecompressionFeature(
    CompressionFormat::BZip2,
  )))
}

// =============================================================================
// Sparse
// =============================================================================

#[cfg(feature = "sparse")]
pub fn decompress_sparse(buffer: &[u8], output: &mut [u8]) -> Result<usize> {
  let mut stream: storm_sparse::Decompress = storm_sparse::Decompress::new();

  stream
    .decompress(buffer, output)
    .map_err(|error| Error::new_std(ErrorKind::DecompressionFailure, error))?;

  Ok(stream.total_out())
}

#[cfg(not(feature = "sparse"))]
pub fn decompress_sparse(_buffer: &[u8], _output: &mut [u8]) -> Result<usize> {
  Err(Error::new(ErrorKind::DecompressionFeature(
    CompressionFormat::Sparse,
  )))
}

// =============================================================================
// IMA ADPCM
// =============================================================================

#[cfg(feature = "adpcm")]
pub fn decompress_adpcm(buffer: &[u8], output: &mut [u8], channels: usize) -> Result<usize> {
  let mut stream: storm_adpcm::Decompress = storm_adpcm::Decompress::new(channels);

  stream
    .decompress(buffer, output)
    .map_err(|error| Error::new_std(ErrorKind::DecompressionFailure, error))?;

  Ok(stream.total_out())
}

#[cfg(not(feature = "adpcm"))]
pub fn decompress_adpcm(_buffer: &[u8], _output: &mut [u8], channels: usize) -> Result<usize> {
  if channels == 1 {
    Err(Error::new(ErrorKind::DecompressionFeature(
      CompressionFormat::Adpcm(false),
    )))
  } else {
    Err(Error::new(ErrorKind::DecompressionFeature(
      CompressionFormat::Adpcm(true),
    )))
  }
}

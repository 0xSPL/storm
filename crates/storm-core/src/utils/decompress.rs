use crate::consts;
use crate::error::Error;
use crate::error::ErrorKind;
use crate::error::Result;

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
// Huffman
// =============================================================================

#[cfg(feature = "huffman")]
pub fn decompress_huffman(buffer: &[u8], output: &mut [u8]) -> Result<usize> {
  panic!("decompress_huffman");
}

#[cfg(not(feature = "huffman"))]
pub fn decompress_huffman(_buffer: &[u8], _output: &mut [u8]) -> Result<usize> {
  Err(Error::new(ErrorKind::DecompressionFeature("huffman", "Huffman Coding")))
}

// =============================================================================
// ZLib
// =============================================================================

#[cfg(feature = "zlib")]
pub fn decompress_zlib(buffer: &[u8], output: &mut [u8]) -> Result<usize> {
  panic!("decompress_zlib");
}

#[cfg(not(feature = "zlib"))]
pub fn decompress_zlib(_buffer: &[u8], _output: &mut [u8]) -> Result<usize> {
  Err(Error::new(ErrorKind::DecompressionFeature("zlib", "ZLib Compression")))
}

// =============================================================================
// PKWare
// =============================================================================

#[cfg(feature = "pkware")]
pub fn decompress_pkware(buffer: &[u8], output: &mut [u8]) -> Result<usize> {
  panic!("decompress_pkware");
}

#[cfg(not(feature = "pkware"))]
pub fn decompress_pkware(_buffer: &[u8], _output: &mut [u8]) -> Result<usize> {
  Err(Error::new(ErrorKind::DecompressionFeature("pkware", "PKWare Compression")))
}

// =============================================================================
// BZip2
// =============================================================================

#[cfg(feature = "bzip2")]
pub fn decompress_bzip2(buffer: &[u8], output: &mut [u8]) -> Result<usize> {
  panic!("decompress_bzip2");
}

#[cfg(not(feature = "bzip2"))]
pub fn decompress_bzip2(_buffer: &[u8], _output: &mut [u8]) -> Result<usize> {
  Err(Error::new(ErrorKind::DecompressionFeature("bzip2", "BZip2 Compression")))
}

// =============================================================================
// Sparse
// =============================================================================

#[cfg(feature = "sparse")]
pub fn decompress_sparse(_buffer: &[u8], _output: &mut [u8]) -> Result<usize> {
  panic!("decompress_sparse");
}

#[cfg(not(feature = "sparse"))]
pub fn decompress_sparse(_buffer: &[u8], _output: &mut [u8]) -> Result<usize> {
  Err(Error::new(ErrorKind::DecompressionFeature("sparse", "Sparse Compression")))
}

// =============================================================================
// IMA ADPCM
// =============================================================================

#[cfg(feature = "adpcm")]
pub fn decompress_adpcm(buffer: &[u8], output: &mut [u8], channels: usize) -> Result<usize> {
  panic!("decompress_adpcm");
}

#[cfg(not(feature = "adpcm"))]
pub fn decompress_adpcm(_buffer: &[u8], _output: &mut [u8], channels: usize) -> Result<usize> {
  if channels == 1 {
    Err(Error::new(ErrorKind::DecompressionFeature("adpcm", "IMA ADPCM (mono) Compression")))
  } else {
    Err(Error::new(ErrorKind::DecompressionFeature("adpcm", "IMA ADPCM (stereo) Compression")))
  }
}

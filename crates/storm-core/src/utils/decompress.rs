use crate::consts;
use crate::error::Error;
use crate::error::Result;

pub fn decompress(buffer: &[u8], output: &mut [u8]) -> Result<usize> {
  panic!("decompress");
}

pub fn decompress_huffman(buffer: &[u8], output: &mut [u8]) -> Result<usize> {
  panic!("decompress_huffman");
}

pub fn decompress_zlib(buffer: &[u8], output: &mut [u8]) -> Result<usize> {
  panic!("decompress_zlib");
}

pub fn decompress_pkware(buffer: &[u8], output: &mut [u8]) -> Result<usize> {
  panic!("decompress_pkware");
}

pub fn decompress_bzip2(buffer: &[u8], output: &mut [u8]) -> Result<usize> {
  panic!("decompress_bzip2");
}

pub fn decompress_sparse(_buffer: &[u8], _output: &mut [u8]) -> Result<usize> {
  panic!("decompress_sparse");
}

pub fn decompress_adpcm(buffer: &[u8], output: &mut [u8], channels: usize) -> Result<usize> {
  panic!("decompress_adpcm");
}

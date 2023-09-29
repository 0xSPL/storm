use storm_core::error::Result;
use storm_core::types::File;
use storm_utils::traits::Parse;

use crate::bundle::BundleData;
use crate::bundle::BundleImage;
use crate::document::DocumentHeader;
use crate::image::dds;

type ParseFn = fn(File) -> Result<BundleData>;

// =============================================================================
// Known Formats
// =============================================================================

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Format {
  Ends(&'static str),
  Name(&'static str),
}

impl Format {
  const FORMATS: &'static [(Self, Option<ParseFn>)] = &[
    // Standard
    (Format::Ends(".cnd"), None), // Plain Text (Candidate Scenarios)
    (Format::Ends(".wav"), None), // WAVE Sound
    (Format::Ends(".txt"), None), // Plain Text
    // StarCraft II (?)
    (Format::Name("DocumentHeader"), Some(parse_document_header)),
    (Format::Name("DocumentInfo"), None),    // XML
    (Format::Ends(".dds"), Some(parse_dds)), // Texture
    (Format::Ends(".lst"), None),            // (?) - TODO: Inspect
    (Format::Ends(".SC2Components"), None),  // XML
    (Format::Ends(".SC2Layout"), None),      // XML
    (Format::Ends(".SC2Style"), None),       // XML
    (Format::Ends(".version"), None),        // (?) - TODO: Inspect
  ];

  pub fn find(name: &str) -> Option<&'static Self> {
    Self::FORMATS
      .iter()
      .find(|(format, _)| format.matches(name))
      .map(|(format, _)| format)
  }

  pub fn parser(&self) -> Option<ParseFn> {
    Self::FORMATS
      .iter()
      .find(|(name, _)| name == self)
      .map(|(_, func)| func)
      .copied()
      .flatten()
  }

  #[inline]
  fn matches(&self, name: &str) -> bool {
    match self {
      Self::Ends(pattern) => name.ends_with(pattern),
      Self::Name(pattern) => name.eq_ignore_ascii_case(pattern),
    }
  }
}

fn parse_document_header(file: File) -> Result<BundleData> {
  DocumentHeader::from_slice(&file).map(BundleData::DHead)
}

fn parse_dds(file: File) -> Result<BundleData> {
  dds::Image::from_slice(&file)
    .map(BundleImage::Dds)
    .map(BundleData::Image)
}

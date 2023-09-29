use storm_core::types::File;

use crate::document::DocumentHeader;
use crate::image::dds;

feature! {
  #[cfg(feature = "json")]
  use std::fs;
  use std::path::Path;
  use std::path::PathBuf;
  use storm_core::error::Result;
  use storm_utils::traits::ExportJson;
}

// =============================================================================
// Bundle File
// =============================================================================

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct BundleFile {
  pub name: String,
  pub data: BundleData,
}

impl BundleFile {
  #[cfg(feature = "json")]
  pub fn write<P>(&self, path: &P) -> Result<()>
  where
    P: AsRef<Path> + ?Sized,
  {
    let path: PathBuf = path.as_ref().join(&self.name);

    match self.data {
      BundleData::Plain(ref data) => {
        fs::write(path, &data[..])?;
      }
      BundleData::DHead(ref data) => {
        data.export_json(&path)?;
      }
      BundleData::Image(ref data) => match data {
        BundleImage::Dds(ref data) => {
          data.export_png(&path, 0)?;
        }
      },
    }

    Ok(())
  }
}

// =============================================================================
// Bundle File Data
// =============================================================================

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum BundleData {
  Plain(File),
  DHead(DocumentHeader),
  Image(BundleImage),
}

// =============================================================================
// Bundle File Image
// =============================================================================

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum BundleImage {
  Dds(dds::Image),
}

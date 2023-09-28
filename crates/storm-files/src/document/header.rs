use storm_core::error::Error;
use storm_core::types::Magic;
use storm_utils::traits::Parse;
use storm_utils::traits::ReadExt;

use crate::document::Attributes;
use crate::document::Dependencies;

// =============================================================================
// Document Header
// =============================================================================

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct DocumentHeader {
  dependencies: Dependencies,
  attributes: Attributes,
}

impl DocumentHeader {
  /// [`DocumentHeader`] magic signature.
  pub const MAGIC: Magic = unsafe { Magic::new_unchecked(*b"H2CS") };

  /// Returns a reference to the document dependencies.
  pub const fn dependencies(&self) -> &Dependencies {
    &self.dependencies
  }

  /// Returns a reference to the document attributes.
  pub const fn attributes(&self) -> &Attributes {
    &self.attributes
  }
}

impl Parse for DocumentHeader {
  type Error = Error;

  fn from_reader<R: ReadExt + ?Sized>(reader: &mut R) -> Result<Self, Self::Error> {
    let magic: [u8; 4] = reader.read_array_u8()?;

    if Self::MAGIC.as_ref() != magic {
      panic!("Invalid Magic"); // TODO: Proper Error
    }

    let _header: [u8; 40] = reader.read_array_u8()?; // TODO: Parse this
    let dependencies: Dependencies = reader.parse()?;
    let attributes: Attributes = reader.parse()?;

    Ok(Self {
      dependencies,
      attributes,
    })
  }
}

only_serde! {
  use serde::ser::SerializeStruct;
  use serde::Serialize;
  use serde::Serializer;

  impl Serialize for DocumentHeader {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
      let mut state: S::SerializeStruct = serializer.serialize_struct("DocumentHeader", 2)?;
      state.serialize_field("dependencies", self.dependencies())?;
      state.serialize_field("attributes", self.attributes())?;
      state.end()
    }
  }
}

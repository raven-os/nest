//! Provides types and functions to represent a package, a manifest and their metadata.

macro_rules! impl_serde_visitor {
    ($Visited:ident, $Visitor:ident) => {
        impl serde::Serialize for $Visited {
            #[inline]
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                self.to_string().serialize(serializer)
            }
        }

        impl<'a> serde::Deserialize<'a> for $Visited {
            #[inline]
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'a>,
            {
                deserializer.deserialize_str($Visitor)
            }
        }
    };
}

mod error;
mod identification;
mod manifest;
mod metadata;
//mod requirement;

pub use identification::{CategoryName, PackageFullName, PackageID, PackageName, RepositoryName};
pub use manifest::{Kind, Manifest, PackageManifest, VersionData};
pub use metadata::{License, Maintainer, Metadata, Tag, UpstreamURL};

lazy_static::lazy_static! {
    /// A regular expression to match and parse a package's string representation
    static ref REGEX_PACKAGE_ID: regex::Regex = regex::Regex::new(
        r"^(?:(?P<repository>[^:/#]+)::)?(?:(?P<category>[^:/#]+)/)?(?P<package>[^:/#]+)(?:#(?P<version>.+))?$"
    ).unwrap();//expect("Failed to parse regex REGEX_PACKAGE_ID");
}

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

macro_rules! strong_name_impl {
    ($NameType:ident, $RegexValidator:expr, $ParseErrorType:ident) => {
        impl std::convert::TryFrom<&str> for $NameType {
            type Error = $ParseErrorType;

            #[inline]
            fn try_from(value: &str) -> Result<Self, Self::Error> {
                lazy_static! {
                    static ref REGEX: Regex = Regex::new($RegexValidator).unwrap();
                }

                if REGEX.is_match(value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err($ParseErrorType(value.to_string()))
                }
            }
        }

        impl std::ops::Deref for $NameType {
            type Target = String;

            #[inline]
            fn deref(&self) -> &String {
                &self.0
            }
        }

        impl std::convert::AsRef<str> for $NameType {
            #[inline]
            fn as_ref(&self) -> &str {
                self.0.as_ref()
            }
        }

        impl std::fmt::Display for $NameType {
            #[inline]
            fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(fmt, "{}", self.0)
            }
        }
    };
}

mod error;
mod identification;
mod manifest;
mod metadata;
mod npf;
mod requirement;

pub use identification::{
    CategoryName, PackageFullName, PackageID, PackageName, PackageShortName, RepositoryName,
};
pub use manifest::{Kind, Manifest, PackageManifest, VersionData};
pub use metadata::{License, Maintainer, Metadata, Tag, UpstreamURL};
pub use npf::{NPFExplorer, NPFFile};
pub use requirement::{HardPackageRequirement, PackageRequirement};

lazy_static::lazy_static! {
    /// A regular expression to match and parse a package's string representation
    static ref REGEX_PACKAGE_ID: regex::Regex = regex::Regex::new(
        r"^(?:(?P<repository>[^:/#]+)::)?(?:(?P<category>[^:/#]+)/)?(?P<package>[^:/#]+)(?:#(?P<version>.+))?$"
    ).unwrap();
}

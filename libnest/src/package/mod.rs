//! Provides types and functions to interact with all kinds of packages: available ones, installed ones etc.

mod errors;
mod identification;
mod manifest;
mod requirement;

pub use self::errors::*;
pub use self::identification::{PackageFullName, PackageID};
pub use self::manifest::{Manifest, Metadata};
pub use self::requirement::{HardPackageRequirement, PackageRequirement};

use std::fs::File;

use failure::{Error, ResultExt};
use lazy_static::lazy_static;
use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::path::Path;

/// A regular expression to match and parse a package's string representation
lazy_static! {
    static ref REGEX_PACKAGE_ID: Regex = Regex::new(
        r"^((?P<repository>[a-z\-]+)::)?((?P<category>[a-z\-]+)/)?(?P<package>([a-z0-9\-*]+))(#(?P<version>(.+)))?$"
    ).unwrap();
}

/// A package's kind.
///
/// All entities called 'package' may not represent the same thing.
/// Some are actual binaries or libraries like one may expect ('effective' packages), but
/// others may be entirely empty, used only to name a list of dependencies ('virtual' packages).
///
/// The `Kind` enum is used to differentiate those packages and speed up their installation process.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum Kind {
    /// The package contains some installable data.
    Effective,
    /// The package doesn't contain any data.
    Virtual,
}

impl Default for Kind {
    fn default() -> Kind {
        Kind::Effective
    }
}

/// A package's name.
///
/// A `PackageName` can be casted to an `&str` and ensures, when created, that the underlying string matches
/// the expectations of what a package's name should look like.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize)]
pub struct PackageName(String);

impl PackageName {
    /// Create a [`PackageName`] from a String
    pub fn from(name: String) -> Self {
        PackageName(name)
    }
}

impl std::ops::Deref for PackageName {
    type Target = String;

    #[inline]
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::convert::AsRef<Path> for PackageName {
    #[inline]
    fn as_ref(&self) -> &std::path::Path {
        self.0.as_ref()
    }
}

impl std::fmt::Display for PackageName {
    #[inline]
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

/// A category's name.
///
/// A `CategoryName` can be casted to an `&str` and ensures, when created, that the underlying string matches
/// the expectations of what a category's name should look like.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize)]
pub struct CategoryName(String);

impl CategoryName {
    /// Create a [`CategoryName`] from a String
    pub fn from(name: String) -> Self {
        CategoryName(name)
    }
}

impl std::ops::Deref for CategoryName {
    type Target = String;

    #[inline]
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::convert::AsRef<Path> for CategoryName {
    #[inline]
    fn as_ref(&self) -> &std::path::Path {
        self.0.as_ref()
    }
}

impl std::fmt::Display for CategoryName {
    #[inline]
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

/// A repository's name.
///
/// A `RepositoryName` can be casted to an `&str` and ensures, when created, that the underlying string matches
/// the expectations of what a repository's name should look like.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize)]
pub struct RepositoryName(String);

impl RepositoryName {
    /// Create a [`RepositoryName`] from a String
    pub fn from(name: String) -> Self {
        RepositoryName(name)
    }
}

impl std::ops::Deref for RepositoryName {
    type Target = String;

    #[inline]
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::convert::AsRef<Path> for RepositoryName {
    #[inline]
    fn as_ref(&self) -> &std::path::Path {
        self.0.as_ref()
    }
}

impl std::fmt::Display for RepositoryName {
    #[inline]
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

/// One of the possibly many package's tag.
///
/// A `Tag` can be casted to an `&str` and ensures, when created, that the underlying string matches
/// the expectations of what a tag should look like.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize)]
pub struct Tag(String);

impl Tag {
    /// Create a [`Tag`] from a String
    pub fn from(tag: String) -> Self {
        Tag(tag)
    }
}

impl std::fmt::Display for Tag {
    #[inline]
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

/// A handle that encapsulates a manifest and the name of the repository this package belongs to.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Package {
    repository: RepositoryName,
    manifest: Manifest,
}

impl Package {
    /// Creates a new package from a [`Repository`] and a [`Manifest`]
    #[inline]
    pub fn from(repository: RepositoryName, manifest: Manifest) -> Self {
        Package {
            repository,
            manifest,
        }
    }

    #[inline]
    pub(crate) fn load_from_cache<P: AsRef<Path>>(
        repository: RepositoryName,
        cache_path: P,
    ) -> Result<Package, Error> {
        let file =
            File::open(cache_path.as_ref()).context(cache_path.as_ref().display().to_string())?;

        Ok(Package {
            repository,
            manifest: serde_json::from_reader(&file)
                .context(cache_path.as_ref().display().to_string())?,
        })
    }

    /// Returns the name of the repository this package belongs to
    #[inline]
    pub fn repository(&self) -> &RepositoryName {
        &self.repository
    }

    /// Returns the package's manifest
    #[inline]
    pub fn manifest(&self) -> &Manifest {
        &self.manifest
    }

    /// Returns the unique ID of the package
    #[inline]
    pub fn id(&self) -> PackageID {
        PackageID::from(
            PackageFullName::from(
                self.repository().clone(),
                self.manifest.metadata().category().clone(),
                self.manifest.metadata().name().clone(),
            ),
            self.manifest.metadata().version().clone(),
        )
    }

    /// Returns the full name of the package
    #[inline]
    pub fn full_name(&self) -> PackageFullName {
        PackageFullName::from(
            self.repository.clone(),
            self.manifest.metadata().category().clone(),
            self.manifest.metadata().name().clone(),
        )
    }
}

impl std::fmt::Display for Package {
    #[inline]
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.id())
    }
}

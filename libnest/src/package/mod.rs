//! Provides types and functions to interact with all kinds of packages: available ones, installed ones etc.

mod identification;
mod manifest;

pub use self::identification::{PackageFullName, PackageID};
pub use self::manifest::{Manifest, Metadata};

use serde_derive::{Deserialize, Serialize};
use std::path::Path;

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
    pub fn new(repository: RepositoryName, manifest: Manifest) -> Self {
        Package {
            repository,
            manifest,
        }
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
}

impl std::fmt::Display for Package {
    #[inline]
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.id())
    }
}

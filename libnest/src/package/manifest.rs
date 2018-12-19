//! Provides `PackageManifest`, a type representing a package's metadata and dependencies.

use semver::Version;

/// A package's kind.
///
/// All entities called 'package' may not represent the same thing.
/// Some are actual binaries or libraries like one may expect ('effective' packages), but
/// others may be entirely empty, used only to name a list of dependencies ('virtual' packages).
///
/// The `Kind` enum is used to differentiate those packages and speed up their installation process.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
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
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct PackageName(String);

/// A category's name.
///
/// A `CategoryName` can be casted to an `&str` and ensures, when created, that the underlying string matches
/// the expectations of what a category's name should look like.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct CategoryName(String);

/// A repository's name.
///
/// A `RepositoryName` can be casted to an `&str` and ensures, when created, that the underlying string matches
/// the expectations of what a repository's name should look like.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct RepositoryName(String);

/// One of the possibly many package's tag.
///
/// A `Tag` can be casted to an `&str` and ensures, when created, that the underlying string matches
/// the expectations of what a tag should look like.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Tag(String);

/// Represents a package's metadata, like its name, category, description etc.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Metadata {
    name: PackageName,
    category: CategoryName,
    repository: RepositoryName,
    description: String,
    tags: Vec<Tag>,
}

/// Represents a package's manifest. It wraps the package's metadata and the available
/// versions of this package for the current architecture.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Manifest {
    metadata: Metadata,
    kind: Kind,
    versions: Vec<Version>,
}

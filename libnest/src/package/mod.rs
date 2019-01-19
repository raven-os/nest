//! Provides types and functions to interact with all kinds of packages: available ones, installed ones etc.

pub mod identification;
pub mod manifest;

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

impl PackageName {
    /// Create a [`PackageName`] from a String
    pub fn from(name: String) -> Self {
        PackageName(name)
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
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct CategoryName(String);

impl CategoryName {
    /// Create a [`CategoryName`] from a String
    pub fn from(name: String) -> Self {
        CategoryName(name)
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
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct RepositoryName(String);

impl RepositoryName {
    /// Create a [`RepositoryName`] from a String
    pub fn from(name: String) -> Self {
        RepositoryName(name)
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
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
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

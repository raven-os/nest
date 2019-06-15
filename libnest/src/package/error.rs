//! Errors that can be returned by the package module

use failure::{Context, Fail};

/// Type for errors related to the parsing of a [`PackageID`]
#[derive(Debug)]
pub struct PackageIDParseError {
    inner: Context<PackageIDParseErrorKind>,
}

/// Type describing a kind of error related to the parsing of a [`PackageID`]
#[derive(Clone, Eq, PartialEq, Hash, Debug, Fail)]
pub enum PackageIDParseErrorKind {
    /// The given string does not follow the format for package IDs
    #[fail(
        display = "\"{}\" doesn't follow the `repository::category/name#version` format",
        _0
    )]
    InvalidFormat(String),

    /// The name component of the package ID has invalid characters
    #[fail(display = "{}", _0)]
    InvalidName(#[cause] PackageNameParseError),

    /// The category component of the package ID has invalid characters
    #[fail(display = "{}", _0)]
    InvalidCategory(#[cause] CategoryNameParseError),

    /// The repository component of the package ID has invalid characters
    #[fail(display = "{}", _0)]
    InvalidRepository(#[cause] RepositoryNameParseError),

    /// The version component of the package ID is not a valid version
    #[fail(display = "invalid version syntax")]
    InvalidVersion,
}

use_as_error!(PackageIDParseError, PackageIDParseErrorKind);

/// Type for errors related to the parsing of a [`PackageFullName`]
#[derive(Debug)]
pub struct PackageFullNameParseError {
    inner: Context<PackageFullNameParseErrorKind>,
}

/// Type describing a kind of error related to the parsing of a [`PackageFullName`]
#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum PackageFullNameParseErrorKind {
    /// The given string does not follow the format for package full names
    #[fail(
        display = "\"{}\" doesn't follow the `repository::category/name` format",
        _0
    )]
    InvalidFormat(String),

    /// The name component of the package full name has invalid characters
    #[fail(display = "{}", _0)]
    InvalidName(#[cause] PackageNameParseError),

    /// The category component of the package full name has invalid characters
    #[fail(display = "{}", _0)]
    InvalidCategory(#[cause] CategoryNameParseError),

    /// The repository component of the package full name has invalid characters
    #[fail(display = "{}", _0)]
    InvalidRepository(#[cause] RepositoryNameParseError),
}

use_as_error!(PackageFullNameParseError, PackageFullNameParseErrorKind);

/// Type for errors related to the parsing of a [`PackageShortName`]
#[derive(Debug)]
pub struct PackageShortNameParseError {
    inner: Context<PackageShortNameParseErrorKind>,
}

/// Type describing a kind of error related to the parsing of a [`PackageShortName`]
#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum PackageShortNameParseErrorKind {
    /// The given string does not follow the format for package short names
    #[fail(display = "\"{}\" doesn't follow the `category/name` format", _0)]
    InvalidFormat(String),

    /// The name component of the package short name has invalid characters
    #[fail(display = "{}", _0)]
    InvalidName(#[cause] PackageNameParseError),

    /// The category component of the package short name has invalid characters
    #[fail(display = "{}", _0)]
    InvalidCategory(#[cause] CategoryNameParseError),
}

use_as_error!(PackageShortNameParseError, PackageShortNameParseErrorKind);

/// Type for errors related to the parsing of a [`PackageRequirement`]
#[derive(Debug)]
pub struct PackageRequirementParseError {
    inner: Context<PackageRequirementParseErrorKind>,
}

/// Type describing a kind of error related to the parsing of a [`PackageRequirement`]
#[derive(Clone, Eq, PartialEq, Hash, Debug, Fail)]
pub enum PackageRequirementParseErrorKind {
    /// The given string does not follow the format for package requirements
    #[fail(
        display = "\"{}\" doesn't follow the `repository::category/name#version` format",
        _0
    )]
    InvalidFormat(String),

    /// The name component of the package requirement has invalid characters
    #[fail(display = "{}", _0)]
    InvalidName(#[cause] PackageNameParseError),

    /// The category component of the package requirement has invalid characters
    #[fail(display = "{}", _0)]
    InvalidCategory(#[cause] CategoryNameParseError),

    /// The repository component of the package requirement has invalid characters
    #[fail(display = "{}", _0)]
    InvalidRepository(#[cause] RepositoryNameParseError),

    /// The version component of the package requirement is not a valid version
    #[fail(display = "invalid version syntax")]
    InvalidVersion,
}

use_as_error!(
    PackageRequirementParseError,
    PackageRequirementParseErrorKind
);

/// Strong type to represent an error message related to the parsing of a package name
#[derive(Clone, Eq, PartialEq, Hash, Debug, Fail)]
#[fail(display = "{}: invalid package name", 0)]
pub struct PackageNameParseError(pub String);

/// Strong type to represent an error message related to the parsing of a category name
#[derive(Clone, Eq, PartialEq, Hash, Debug, Fail)]
#[fail(display = "{}: invalid category name", 0)]
pub struct CategoryNameParseError(pub String);

/// Strong type to represent an error message related to the parsing of a repository name
#[derive(Clone, Eq, PartialEq, Hash, Debug, Fail)]
#[fail(display = "{}: invalid repository name", 0)]
pub struct RepositoryNameParseError(pub String);

/// Strong type to represent an error message related to the parsing of a package tag
#[derive(Clone, Eq, PartialEq, Hash, Debug, Fail)]
#[fail(display = "{}: invalid tag", 0)]
pub struct TagParseError(pub String);

/// Strong type to represent an error message related to the parsing of a package license
#[derive(Clone, Eq, PartialEq, Hash, Debug, Fail)]
#[fail(display = "{}: invalid license", 0)]
pub struct LicenseParseError(pub String);

/// Strong type to represent an error message related to the parsing of a package slot
#[derive(Clone, Eq, PartialEq, Hash, Debug, Fail)]
#[fail(display = "{}: invalid slot", 0)]
pub struct SlotParseError(pub String);

/// Type for errors related to the exploration of an NPF file
#[derive(Debug)]
pub struct NPFExplorationError {
    inner: Context<NPFExplorationErrorKind>,
}

/// Kind for errors related to the exploration of an NPF file
#[derive(Clone, Eq, PartialEq, Hash, Debug, Fail)]
pub enum NPFExplorationErrorKind {
    /// An NPF file could not be unpacked for exploration
    #[fail(display = "unable to unpack")]
    UnpackError,

    /// The mandatory manifest file for an NPF could not be found
    #[fail(display = "the manifest.toml file could not be found")]
    MissingManifest,

    /// The mandatory manifest file for an NPF was found, but is invalid
    #[fail(display = "invalid manifest.toml")]
    InvalidManifest,

    /// A requested file could not be found in the NPF
    #[fail(display = "the requested file not found in the NPF: {:?}", _0)]
    FileNotFound(std::path::PathBuf),

    #[fail(
        display = "the requested file in the NPF could not be opened: {:?}",
        _0
    )]
    /// A requested file was found in an NPF, but could not be used
    FileIOError(std::path::PathBuf),
}

use_as_error!(NPFExplorationError, NPFExplorationErrorKind);

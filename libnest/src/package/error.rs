//! Errors that can be returned by the package module

use failure::{Context, Fail};

#[derive(Debug)]
pub struct PackageIDParseError {
    inner: Context<PackageIDParseErrorKind>,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Fail)]
pub enum PackageIDParseErrorKind {
    #[fail(
        display = "\"{}\" doesn't follow the `repository::category/name#version` format",
        _0
    )]
    InvalidFormat(String),
    #[fail(display = "{}", _0)]
    InvalidName(#[cause] PackageNameParseError),
    #[fail(display = "{}", _0)]
    InvalidCategory(#[cause] CategoryNameParseError),
    #[fail(display = "{}", _0)]
    InvalidRepository(#[cause] RepositoryNameParseError),
    #[fail(display = "invalid version syntax")]
    InvalidVersion,
}

use_as_error!(PackageIDParseError, PackageIDParseErrorKind);

#[derive(Debug)]
pub struct PackageFullNameParseError {
    inner: Context<PackageFullNameParseErrorKind>,
}

#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum PackageFullNameParseErrorKind {
    #[fail(
        display = "\"{}\" doesn't follow the `repository::category/name` format",
        _0
    )]
    InvalidFormat(String),
    #[fail(display = "{}", _0)]
    InvalidName(#[cause] PackageNameParseError),
    #[fail(display = "{}", _0)]
    InvalidCategory(#[cause] CategoryNameParseError),
    #[fail(display = "{}", _0)]
    InvalidRepository(#[cause] RepositoryNameParseError),
}

use_as_error!(PackageFullNameParseError, PackageFullNameParseErrorKind);

#[derive(Debug)]
pub struct PackageShortNameParseError {
    inner: Context<PackageShortNameParseErrorKind>,
}

#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum PackageShortNameParseErrorKind {
    #[fail(display = "\"{}\" doesn't follow the `category/name` format", _0)]
    InvalidFormat(String),
    #[fail(display = "{}", _0)]
    InvalidName(#[cause] PackageNameParseError),
    #[fail(display = "{}", _0)]
    InvalidCategory(#[cause] CategoryNameParseError),
}

use_as_error!(PackageShortNameParseError, PackageShortNameParseErrorKind);

#[derive(Clone, Eq, PartialEq, Hash, Debug, Fail)]
#[fail(display = "{}: invalid package name", 0)]
pub struct PackageNameParseError(pub String);

#[derive(Clone, Eq, PartialEq, Hash, Debug, Fail)]
#[fail(display = "{}: invalid category name", 0)]
pub struct CategoryNameParseError(pub String);

#[derive(Clone, Eq, PartialEq, Hash, Debug, Fail)]
#[fail(display = "{}: invalid repository name", 0)]
pub struct RepositoryNameParseError(pub String);

#[derive(Clone, Eq, PartialEq, Hash, Debug, Fail)]
#[fail(display = "{}: invalid tag", 0)]
pub struct TagParseError(pub String);

#[derive(Clone, Eq, PartialEq, Hash, Debug, Fail)]
#[fail(display = "{}: invalid slot", 0)]
pub struct SlotParseError(pub String);

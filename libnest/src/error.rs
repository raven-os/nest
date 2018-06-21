//! Types and enum for error handling, using [`failure`][1].
//!
//! [1]: https://docs.rs/failure/0.1.1/failure/

/// Errors that may occure when pulling a repository
#[derive(Clone, PartialEq, Debug, Fail)]
pub enum PullError {
    /// The error occured while interpreting the received data
    #[fail(display = "received invalid data")]
    InvalidData,
    /// The removal of the old cache failed
    #[fail(display = "can't remove old cache")]
    CantRemoveCache,
    /// The update of the cache failed
    #[fail(display = "can't update cache of package {}", _0)]
    CantUpdateCache(String),
}

/// Errors that may occure when installing a package
#[derive(Clone, PartialEq, Debug, Fail)]
pub enum InstallError {
    /// The package is already installed, therefore it can't be installed twice.
    #[fail(display = "\"{}\" is already installed", _0)]
    PackageAlreadyInstalled(String),
    /// One of the installed file already exist on the filesystem, and libnest won't replace it.
    #[fail(display = "\"{}\" already exists", _0)]
    FileAlreadyExists(String),
    /// The user is trying to install or update a package where it's repository is no longer in the configuration file.
    #[fail(display = "the repository \"{}\" of package \"{}\" can't be found", _0, _1)]
    CantFindRepository(String, String),
}

/// Errors that may occure when parsing a package requirement.
#[derive(Debug, Fail)]
pub enum PackageRequirementParseError {
    /// The package's name is invalid
    #[fail(display = "invalid package name \"{}\"", _0)]
    InvalidPackageName(String),
    /// The version requirement is invalid
    #[fail(display = "invalid version requirement in package name \"{}\"", _0)]
    InvalidVersionRequirement(String),
}

/// Errors that may occure when manipulating the dependency graph.
#[derive(Debug, Fail)]
pub enum DepGraphErrorKind {
    /// The [`NodeId`] doesn't match any [`Node`] inside the [`DependencyGraph`].
    #[fail(display = "invalid node id")]
    InvalidNodeId,
    /// Can't find a package that matches the given requirement.
    #[fail(display = "can't find a package meeting the requirement \"{}\"", _0)]
    CantFindPackage(String),
}

/// Errors that may occure when downloading files from a [`Repository`].
#[derive(Debug, Fail)]
pub enum TransferError {
    /// The transfer failed because all mirrors are down
    #[fail(display = "all mirrors are down")]
    AllMirrorsDown,
}

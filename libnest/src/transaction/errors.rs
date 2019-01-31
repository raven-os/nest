//! Errors that can be returned by the transaction module

use failure::{Context, Fail};

/// Error type for errors related to package installation
#[derive(Debug)]
pub struct InstallError {
    inner: Context<InstallErrorKind>,
}

/// Error kind describing a kind of error related to package installation
#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum InstallErrorKind {
    /// The package could not be installed because it would overwrite an existing file
    #[fail(display = "file already exists")]
    FileAlreadyExists,

    /// The package could not be installed because it is already installed
    #[fail(display = "package already installed")]
    PackageAlreadyInstalled,
}

use_as_error!(InstallError, InstallErrorKind);

/// Error type for errors related to package removal
#[derive(Debug)]
pub struct RemoveError {
    inner: Context<RemoveErrorKind>,
}

/// Error kind describing a kind of error related to package removal
#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum RemoveErrorKind {
    /// The package could not be removed because its log file could not be loaded
    #[fail(display = "log file not found")]
    LogFileLoadError,

    /// The package could not be completely removed because one of its files could not be removed
    #[fail(display = "cannot remove package file")]
    FileRemoveError,

    /// The package could not be completely removed because its log file could not be removed
    #[fail(display = "cannot remove log file")]
    LogFileRemoveError,
}

use_as_error!(RemoveError, RemoveErrorKind);

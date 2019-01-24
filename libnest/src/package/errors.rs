//! Errors that can be returned by the package module

use failure::{Context, Fail};

/// Error type for package-related errors
#[derive(Debug)]
pub struct PackageError {
    inner: Context<PackageErrorKind>,
}

/// Error kind describing a kind of package-related error
#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum PackageErrorKind {
    /// A package requirement was invalid
    #[fail(display = "invalid package requirement")]
    InvalidPackageRequirement,
}

use_as_error!(PackageError, PackageErrorKind);

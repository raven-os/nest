//! Types and enum for error handling, using [`failure`][1].
//!
//! [1]: https://docs.rs/failure/0.1.1/failure/

use std::fmt::{self, Display, Formatter};

use failure::{Backtrace, Context, Fail};
use toml;
use url::Url;

/// Kind of errors that may occur when using the manifests cache.
#[derive(Clone, PartialEq, Debug, Fail)]
pub enum CacheErrorKind {
    /// The error occured doing an IO operation.
    #[fail(display = "{}", _0)]
    IO(String),
    /// The error occured when deserializing a manifest.
    #[fail(display = "{}", _0)]
    Deserialize(String),
    /// The error occured when serializing a manifest.
    #[fail(display = "{}", _0)]
    Serialize(String),
}

/// A type for errors that may occur when using the manifest's cache.
#[derive(Debug)]
pub struct CacheError {
    inner: Context<CacheErrorKind>,
}

impl CacheError {
    /// Returns a [`CacheErrorKind`] the reason why this error was thrown.
    pub fn kind(&self) -> &CacheErrorKind {
        &self.inner.get_context()
    }
}

impl Fail for CacheError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for CacheError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl From<CacheErrorKind> for CacheError {
    fn from(kind: CacheErrorKind) -> CacheError {
        CacheError {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<CacheErrorKind>> for CacheError {
    fn from(inner: Context<CacheErrorKind>) -> CacheError {
        CacheError { inner }
    }
}

/// Kind of errors that may occur when pulling a repository.
#[derive(Clone, PartialEq, Debug, Fail)]
pub enum PullErrorKind {
    /// The error is network-based and occured while downloading the data.
    #[fail(display = "{}", _0)]
    Download(String),
    /// The error occured while interpreting the received data.
    #[fail(display = "{}: received invalid data", _0)]
    InvalidData(Url),
    /// The error occured while trying to remove the old cache.
    #[fail(display = "can't remove old cache: {}", _0)]
    CantRemoveCache(String),
    /// The error occured while trying to create the new cache.
    #[fail(display = "can't create cache: {}", _0)]
    CantCreateCache(String),
}

/// A type for errors that may occur when pulling a repository.
#[derive(Debug)]
pub struct PullError {
    inner: Context<PullErrorKind>,
}

impl Fail for PullError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for PullError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl From<PullErrorKind> for PullError {
    fn from(kind: PullErrorKind) -> PullError {
        PullError {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<PullErrorKind>> for PullError {
    fn from(inner: Context<PullErrorKind>) -> PullError {
        PullError { inner }
    }
}

/// Kind of errors that may occur when downloading a package from a repository.
#[derive(Clone, PartialEq, Debug, Fail)]
pub enum DownloadErrorKind {
    /// The error is network-based and occured while downloading the data.
    #[fail(display = "{}", _0)]
    Download(String),
}

/// A type for errors that may occur when downloading a package from a repository.
#[derive(Debug)]
pub struct DownloadError {
    inner: Context<DownloadErrorKind>,
}

impl Fail for DownloadError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for DownloadError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl From<DownloadErrorKind> for DownloadError {
    fn from(kind: DownloadErrorKind) -> DownloadError {
        DownloadError {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<DownloadErrorKind>> for DownloadError {
    fn from(inner: Context<DownloadErrorKind>) -> DownloadError {
        DownloadError { inner }
    }
}

/// Kind of errors that may occur when installing a package.
#[derive(Debug, Fail)]

pub enum InstallErrorKind {
    /// One of the installed file already exists on the targeted system.
    #[fail(display = "{} already exists", _0)]
    FileAlreadyExists(String),
    /// The destination directory isn't valid (either does not exist or is not a directory).
    #[fail(display = "\"{}\" either does not exist or is not a directory", _0)]
    DestFolderError(String),
    /// The package is already installed.
    #[fail(display = "the package is already installed")]
    PackageAlreadyInstalled,
}

/// A type for errors that may occur when installing a package.
#[derive(Debug)]
pub struct InstallError {
    inner: Context<InstallErrorKind>,
}

impl Fail for InstallError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for InstallError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl From<InstallErrorKind> for InstallError {
    fn from(kind: InstallErrorKind) -> InstallError {
        InstallError {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<InstallErrorKind>> for InstallError {
    fn from(inner: Context<InstallErrorKind>) -> InstallError {
        InstallError { inner }
    }
}

/// The kind of a [`ConfigLoadError`][1].
///
/// [1]: struct.ConfigLoadError.html
// XXX The display implementation for this enum members aren't used. Instead, QueryError implements a long, nice and complete error message.
#[derive(Debug, Fail)]
pub enum ConfigLoadErrorKind {
    /// The error is caused by an invalid config file that couldn't be deserialized.
    #[fail(display = "couldn't deserialize {}", _0)]
    Deserialize(String, #[cause] toml::de::Error),
}

/// Errors that may occur when querying manifests.
#[derive(Debug)]
pub struct ConfigLoadError {
    inner: Context<ConfigLoadErrorKind>,
}

impl Fail for ConfigLoadError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for ConfigLoadError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl From<ConfigLoadErrorKind> for ConfigLoadError {
    fn from(kind: ConfigLoadErrorKind) -> ConfigLoadError {
        ConfigLoadError {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ConfigLoadErrorKind>> for ConfigLoadError {
    fn from(inner: Context<ConfigLoadErrorKind>) -> ConfigLoadError {
        ConfigLoadError { inner }
    }
}

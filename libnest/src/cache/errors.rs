//! Errors that can be returned by the cache module

use failure::{Context, Fail};

/// Error type for cache-related errors
#[derive(Debug)]
pub struct CacheError {
    inner: Context<CacheErrorKind>,
}

/// Error kind describing a kind of cache-related error
#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum CacheErrorKind {
    /// The data in the cache directory could not be loaded
    #[fail(display = "unable to load the cache")]
    CacheLoadError,

    /// The data in the cache directory could not be parsed
    #[fail(display = "unable to parse the cache file")]
    CacheParseError,

    /// Some data could not be written to the cache
    #[fail(display = "unable to write data to the cache")]
    CacheWriteError,

    /// Some data could not be cleared from the cache
    #[fail(display = "unable to clear data from the cache")]
    CacheClearError,
}

use_as_error!(CacheError, CacheErrorKind);

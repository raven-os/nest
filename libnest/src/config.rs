//! Nest configuration parsing and handle.
//!
//! There is two way to configure operations using the Nest package manager: globally
//! (configuration file), or locally (command line arguments).
//!
//! Within the `libnest`, many functions take a `&Config` as argument. The main reason is to allow local options to be used only for one operation, even in an
//! asynchronous context.
//!
//! This module provide a `Config` structure that holds all configuration options. This includes,
//! for exemple, proxy settings, cache path, mirrors etc.
//!
//! It also provides a way to load a `Config` from a TOML file.

use std::convert::TryFrom;
use std::path::{Path, PathBuf};

use curl;
use curl::easy::Easy;

use repository::Repository;

lazy_static! {
    static ref NEST_PATH_CACHE: &'static Path = Path::new("/var/lib/nest/cache/");
    static ref NEST_PATH_DOWNLOAD: &'static Path = Path::new("/var/lib/nest/download/");
}

/// A handle to represent a configuration for Nest.
///
/// This handle is given as parameter to each libnest's function so they can use a custom configuration even in an asychronous context.
///
/// Configuration includes proxy settings, cache path, repositories and their mirrors etc.
///
/// # Examples
///
/// ```
/// # extern crate libnest;
/// use libnest::config::Config;
///
/// let config = Config::new();
/// ```
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Config {
    cache: PathBuf,
    download_path: PathBuf,
    repositories: Vec<Repository>,
}

impl Config {
    /// Creates a default configuration.
    ///
    /// The default configuration is:
    /// * Cache path: `/var/lib/nest/cache/`
    /// * Download path: `/var/lib/nest/download/`
    ///
    /// All other fields are empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
    /// use libnest::config::Config;
    ///
    /// let config = Config::new();
    /// ```
    #[inline]
    pub fn new() -> Config {
        Config {
            cache: PathBuf::from(*NEST_PATH_CACHE),
            download_path: PathBuf::from(*NEST_PATH_DOWNLOAD),
            repositories: Vec::new(),
        }
    }

    /// Returns the path holding the cache of each repository.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
    /// use std::path::Path;
    /// use libnest::config::Config;
    ///
    /// let config = Config::new();
    /// assert_eq!(config.cache(), Path::new("/var/lib/nest/cache/"));
    /// ```
    #[inline]
    pub fn cache(&self) -> &Path {
        &self.cache
    }

    /// Returns the path where packages's datas are downloaded.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
    /// use std::path::Path;
    /// use libnest::config::Config;
    ///
    /// let config = Config::new();
    /// assert_eq!(config.download_path(), Path::new("/var/lib/nest/download/"));
    /// ```
    pub fn download_path(&self) -> &Path {
        &self.download_path
    }

    /// Yields a reference to the underlying `Vec<Repository>`
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
    /// use libnest::config::Config;
    /// use libnest::repository::Repository;
    ///
    /// let mut config = Config::new();
    /// let repo = Repository::new("local");
    /// assert!(config.repositories().is_empty());
    /// ```
    #[inline]
    pub fn repositories(&self) -> &Vec<Repository> {
        &self.repositories
    }

    /// Yields a mutable reference to the underlying `Vec<Repository>`
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
    /// use libnest::config::Config;
    /// use libnest::repository::Repository;
    ///
    /// let mut config = Config::new();
    /// let repo = Repository::new("local");
    ///
    /// let repos = config.repositories_mut();
    ///
    /// assert!(repos.is_empty());
    /// repos.push(repo);
    /// assert_eq!(repos.len(), 1);
    /// ```
    #[inline]
    pub fn repositories_mut(&mut self) -> &mut Vec<Repository> {
        &mut self.repositories
    }
}

impl Default for Config {
    #[inline]
    fn default() -> Self {
        Config::new()
    }
}

impl<'a> TryFrom<&'a Config> for Easy {
    type Error = curl::Error;

    /// Tries to create a curl handle with the given configuration.
    fn try_from(_: &'a Config) -> Result<Easy, curl::Error> {
        Ok(Easy::new())
    }
}

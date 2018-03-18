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

use std::path::{Path, PathBuf};
use std::convert::TryFrom;

use curl;
use curl::easy::Easy;

use repository::Repository;

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
    repositories: Vec<Repository>,
}

impl Config {
    /// Creates a default configuration.
    ///
    /// The default configuration is:
    /// * Cache path: `/var/lib/nest/cache/`
    ///
    /// All other fields are empty.
    ///
    /// Example:
    /// ```
    /// # extern crate libnest;
    /// use libnest::config::Config;
    ///
    /// let config = Config::new();
    /// ```
    #[inline]
    pub fn new() -> Config {
        Config {
            cache: PathBuf::from("/var/lib/nest/cache/"),
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
    ///
    /// assert_eq!(config.cache(), Path::new("/var/lib/nest/cache"));
    /// ```
    #[inline]
    pub fn cache(&self) -> &Path {
        &self.cache
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
    /// let repo = Repository::new(&config, "local");
    ///
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
    /// let repo = Repository::new(&config, "local");
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

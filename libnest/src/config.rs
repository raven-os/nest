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

use config_parser::{ConfigParser, ParseConfError};
use repository::Repository;

static DEFAULT_CACHE_DIR: &'static str = "/var/lib/nest/cache/";
static DEFAULT_DOWNLOAD_DIR: &'static str = "/var/lib/nest/download/";

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
        Config::default()
    }

    /// Loads a configuration from a TOML file
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
    /// use libnest::config::Config;
    /// use std::path::Path;
    ///
    /// let mut config = Config::new();
    /// config.load(Path::new("Config.toml"));
    /// ```
    pub fn load(&mut self, path: &Path) -> Option<ParseConfError> {
        match ConfigParser::new(path) {
            Ok(conf_parser) => {
                conf_parser.load_to_config(self);
                None
            }
            Err(e) => Some(e),
        }
    }

    /// Returns the path holding the cache of each repository.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
    ///
    /// use std::path::Path;
    /// use libnest::config::Config;
    ///
    /// let config = Config::new();
    /// assert_eq!(config.cache(), Path::new("/var/lib/nest/cache"));
    /// ```
    #[inline]
    pub fn cache(&self) -> &Path {
        &self.cache
    }
    /// Returns the path where packages are downloaded.
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
    #[inline]
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
    /// let repo = Repository::new(&config, "local");
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

    #[inline]
    pub(crate) fn set_cache(&mut self, cache: PathBuf) {
        self.cache = cache;
    }

    #[inline]
    pub(crate) fn set_download_path(&mut self, download: PathBuf) {
        self.download_path = download;
    }

    #[inline]
    pub(crate) fn set_repositories(&mut self, repos: Vec<Repository>) {
        self.repositories = repos;
    }
}

impl Default for Config {
    #[inline]
    fn default() -> Self {
        Config {
            cache: PathBuf::from(DEFAULT_CACHE_DIR),
            download_path: PathBuf::from(DEFAULT_DOWNLOAD_DIR),
            repositories: Vec::new(),
        }
    }
}

impl<'a> TryFrom<&'a Config> for Easy {
    type Error = curl::Error;

    /// Tries to create a curl handle with the given configuration.
    fn try_from(_: &'a Config) -> Result<Easy, curl::Error> {
        Ok(Easy::new())
    }
}

//! Nest configuration parsing and handle.
//!
//! There are two ways to configure operations using the Nest package manager: globally
//! (configuration file), or locally (command line arguments).
//!
//! Within the `libnest`, many functions take a `&Config` as argument. The main reason is to allow local options to be used only for one operation, even in an
//! asynchronous context.
//!
//! This module provides a `Config` structure that holds all configuration options. This includes,
//! for exemple, proxy settings, cache path, mirrors, etc.
//!
//! It also provides a way to load a `Config` from a TOML file.

use std::convert::TryFrom;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use curl;
use curl::easy::Easy;
use failure::{Error, ResultExt};
use toml;

use error::ConfigLoadErrorKind;
use repository::Repository;

lazy_static! {
    static ref NEST_PATH_CONFIG: &'static Path = Path::new("/etc/nest/config.toml");
    static ref NEST_PATH_CACHE: &'static Path = Path::new("/var/nest/cache/");
    static ref NEST_PATH_DOWNLOAD: &'static Path = Path::new("/var/nest/download/");
    static ref NEST_PATH_INSTALLED: &'static Path = Path::new("/var/nest/installed/");
}

/// A structure holding all important paths for libnest. It's a sub member of [`Config`][1].
///
/// [1]: struct.Config.html
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
struct ConfigPaths {
    cache: PathBuf,
    download: PathBuf,
    installed: PathBuf,
}

/// A handle to represent a configuration for Nest.
///
/// This handle is given as parameter to each libnest's function so they can use a custom configuration even in an asychronous context.
///
/// Configuration includes proxy settings, cache path, repositories and their mirrors, etc.
///
/// # Examples
///
/// ```
/// # extern crate libnest;
/// use libnest::config::Config;
///
/// let config = Config::new();
/// ```
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct Config {
    paths: ConfigPaths,
    repositories: Vec<Repository>,
}

impl Config {
    /// Creates a default configuration.
    ///
    /// The default configuration is:
    /// * Cache path: `/var/nest/cache/`
    /// * Download path: `/var/nest/download/`
    /// * Installed path: `/var/nest/installed/`
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
            paths: ConfigPaths {
                cache: PathBuf::from(*NEST_PATH_CACHE),
                download: PathBuf::from(*NEST_PATH_DOWNLOAD),
                installed: PathBuf::from(*NEST_PATH_INSTALLED),
            },
            repositories: Vec::new(),
        }
    }

    /// Loads the default config file.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate libnest;
    /// # use std::error::Error;
    /// # fn test() -> Result<(), Box<Error>> {
    /// use libnest::config::Config;
    ///
    /// let config = Config::load()?;
    /// # Ok(()) }
    /// # fn main() { test(); }
    /// ```
    #[inline]
    pub fn load() -> Result<Config, Error> {
        Config::load_from(*NEST_PATH_CONFIG)
    }

    /// Loads the given config file.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate libnest;
    /// # use std::error::Error;
    /// # fn test() -> Result<(), Box<Error>> {
    /// use libnest::config::Config;
    ///
    /// let config = Config::load_from("./config.toml")?;
    /// # Ok(()) }
    /// # fn main() { test(); }
    /// ```
    #[inline]
    pub fn load_from<P: AsRef<Path>>(path: P) -> Result<Config, Error> {
        let path = path.as_ref();
        let mut file = File::open(path).context(path.display().to_string())?;

        // Allocate a string long enough to hold the entire file
        let mut s = file.metadata()
            .map(|m| String::with_capacity(m.len() as usize))
            .unwrap_or_default();

        file.read_to_string(&mut s)?;
        toml::from_str(&s).map_err(|err| {
            Error::from(ConfigLoadErrorKind::Deserialize(
                path.display().to_string(),
                err,
            ))
        })
    }

    /// Returns a reference to the path where packages' metadata are cached.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
    /// use std::path::Path;
    /// use libnest::config::Config;
    ///
    /// let config = Config::new();
    /// assert_eq!(config.cache(), Path::new("/var/nest/cache/"));
    /// ```
    #[inline]
    pub fn cache(&self) -> &Path {
        &self.paths.cache
    }

    /// Returns a mutable reference to the path where packages' metadata are cached.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
    /// use std::path::{Path, PathBuf};
    /// use libnest::config::Config;
    ///
    /// let mut config = Config::new();
    /// *config.cache_mut() = PathBuf::from("/tmp/cache/");
    /// assert_eq!(config.cache(), Path::new("/tmp/cache"));
    /// ```
    #[inline]
    pub fn cache_mut(&mut self) -> &mut PathBuf {
        &mut self.paths.cache
    }

    /// Returns a reference to the path where packages' data are downloaded.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
    /// use std::path::Path;
    /// use libnest::config::Config;
    ///
    /// let config = Config::new();
    /// assert_eq!(config.download(), Path::new("/var/nest/download/"));
    /// ```
    pub fn download(&self) -> &Path {
        &self.paths.download
    }

    /// Returns a mutable reference to the path where packages' data are downloaded.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
    /// use std::path::{Path, PathBuf};
    /// use libnest::config::Config;
    ///
    /// let mut config = Config::new();
    /// *config.download_mut() = PathBuf::from("/tmp/download/");
    /// assert_eq!(config.download(), Path::new("/tmp/download/"));
    /// ```
    pub fn download_mut(&mut self) -> &mut PathBuf {
        &mut self.paths.download
    }

    /// Returns a reference to the path where installed packages are logged.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
    /// use std::path::Path;
    /// use libnest::config::Config;
    ///
    /// let config = Config::new();
    /// assert_eq!(config.installed(), Path::new("/var/nest/installed/"));
    /// ```
    #[inline]
    pub fn installed(&self) -> &Path {
        &self.paths.installed
    }

    /// Returns a mutable reference to the path where packages' metadata are cached.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
    /// use std::path::{Path, PathBuf};
    /// use libnest::config::Config;
    ///
    /// let mut config = Config::new();
    /// *config.installed_mut() = PathBuf::from("/tmp/installed/");
    /// assert_eq!(config.installed(), Path::new("/tmp/installed/"));
    /// ```
    #[inline]
    pub fn installed_mut(&mut self) -> &mut PathBuf {
        &mut self.paths.installed
    }

    /// Yields a reference to the underlying `Vec<Repository>`.
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

    /// Yields a mutable reference to the underlying `Vec<Repository>`.
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
        let mut curl = Easy::new();
        curl.follow_location(true)?;
        curl.fail_on_error(true)?;
        Ok(curl)
    }
}

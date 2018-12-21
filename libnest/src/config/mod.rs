//! Nest configuration parsing and handle.
//!
//! There are two way sto configure operations using the Nest package manager:
//! globally (using the configuration file), or locally (through command line arguments).
//!
//! Within the `libnest`, many functions take a `&Config` as argument.
//! The main reason is to allow local options to be used only for one operation, even in an
//! asynchronous context.
//!
//! This module provides a `Config` structure that holds all configuration options. This includes,
//! for example, proxy settings, cache path, mirrors etc.
//!
//! It also provides a way to load a `Config` from a TOML file.

mod paths;
mod repository;

pub use self::paths::ConfigPaths;
pub use self::repository::RepositoryConfig;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use failure::{Error, ResultExt};
use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};
use toml;

lazy_static! {
    static ref NEST_PATH_CONFIG: &'static Path = Path::new("/etc/nest/config.toml");
}

/// A handle to represent a configuration for Nest.
///
/// This handle is given as parameter to each libnest function so they can use a custom configuration even in an asynchronous context.
///
/// Configuration includes proxy settings, cache path, repositories and their mirrors etc.
///
/// # Examples
///
/// ```no_run
/// # extern crate libnest;
/// # extern crate failure;
/// # fn main() -> Result<(), failure::Error> {
/// use libnest::config::Config;
///
/// let config = Config::load()?;
/// # Ok(()) }
/// ```
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    paths: ConfigPaths,
    #[serde(default)]
    repositories: HashMap<String, RepositoryConfig>,
}

impl Config {
    /// Loads the configuration located at the default path
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate libnest;
    /// # extern crate failure;
    /// # fn main() -> Result<(), failure::Error> {
    /// use libnest::config::Config;
    ///
    /// let config = Config::load()?;
    /// # Ok(()) }
    /// ```
    #[inline]
    pub fn load() -> Result<Config, Error> {
        Config::load_from(*NEST_PATH_CONFIG)
    }

    /// Loads the configuration file located at the given path
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate libnest;
    /// # extern crate failure;
    /// # fn main() -> Result<(), failure::Error> {
    /// use libnest::config::Config;
    ///
    /// let config = Config::load_from("./config.toml")?;
    /// # Ok(()) }
    /// ```
    #[inline]
    pub fn load_from<P: AsRef<Path>>(path: P) -> Result<Config, Error> {
        let path = path.as_ref();
        let mut file = File::open(path).with_context(|_| path.display().to_string())?;

        // Allocate a string long enough to hold the entire file
        let mut s = file
            .metadata()
            .map(|m| String::with_capacity(m.len() as usize))
            .unwrap_or_default();

        file.read_to_string(&mut s)?;
        Ok(toml::from_str(&s).with_context(|_| path.display().to_string())?)
    }

    /// Returns a reference to an intermediate structure holding all important paths that are used by `libnest`.
    #[inline]
    pub fn paths(&self) -> &ConfigPaths {
        &self.paths
    }

    /// Returns a mutable reference to an intermediate structure holding all important paths that are used by `libnest`.
    #[inline]
    pub fn paths_mut(&mut self) -> &mut ConfigPaths {
        &mut self.paths
    }

    /// Returns a hashmap of mapping a [`RepositoryConfig`] with the name of the repository.
    #[inline]
    pub fn repositories_config(&self) -> &HashMap<String, RepositoryConfig> {
        &self.repositories
    }

    /// Returns a mutable reference of a hashmap of mapping a [`RepositoryConfig`] with the name of the repository.
    #[inline]
    pub fn repositories_config_mut(&mut self) -> &mut HashMap<String, RepositoryConfig> {
        &mut self.repositories
    }
}

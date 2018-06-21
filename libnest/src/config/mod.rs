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

mod paths;
mod repository;
pub use self::paths::ConfigPaths;
pub use self::repository::RepositoryConfig;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use failure::{Error, ResultExt};
use toml;

use cache::available::AvailablePackages;
use cache::depgraph::DependencyGraph;
use repository::Repository;

lazy_static! {
    static ref NEST_PATH_CONFIG: &'static Path = Path::new("/etc/nest/config.toml");
}

/// A handle to represent a configuration for Nest.
///
/// This handle is given as parameter to each libnest's function so they can use a custom configuration even in an asychronous context.
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

    /// Returns a reference an intermediate structure holding all important paths that are used by `libnest`.
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

    /// Returns a vector of all [`Repositories`]
    #[inline]
    pub fn repositories(&self) -> Vec<Repository> {
        self.repositories
            .iter()
            .map(|(name, config)| Repository::from(name, config))
            .collect()
    }

    /// Returns an [`AvailablePackages`], which is a handler over the cache of available packages.
    #[inline]
    pub fn available(&self) -> AvailablePackages {
        AvailablePackages::from(self.paths.available())
    }

    /// Returns, in case of success, a [`DependencyGraph`], which is a handler over the dependency graph, or an [`Error`] in case of failure.
    #[inline]
    pub fn depgraph(&self) -> Result<DependencyGraph, Error> {
        DependencyGraph::load(self.paths.depgraph())
    }
}

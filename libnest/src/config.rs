//! Nest configuration parsing and handle.

use std::path::PathBuf;
use std::slice::Iter;

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
/// extern crate libnest;
///
/// use libnest::config::Config;
///
/// let config = Config::new();
/// ```
#[derive(Debug)]
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
    /// extern crate libnest;
    ///
    /// use libnest::config::Config;
    ///
    /// let config = Config::new();
    /// ```
    pub fn new() -> Config {
        Config {
            cache: PathBuf::from("/var/lib/nest/cache/"),
            repositories: Vec::new(),
        }
    }

    /// Returns the path holding the the cache of each repository.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate libnest;
    ///
    /// use std::path::Path;
    /// use libnest::config::Config;
    ///
    /// let config = Config::new();
    ///
    /// assert_eq!(config.cache(), Path::new("/var/lib/nest/cache"));
    /// ```
    pub fn cache(&self) -> &PathBuf {
        &self.cache
    }

    /// Adds the given repository at the end of the list of repositories, meaning it has the lowest
    /// priority when looking for a package.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate libnest;
    ///
    /// use libnest::config::Config;
    /// use libnest::repository::Repository;
    ///
    /// let mut config = Config::new();
    /// let repo = Repository::new(&config, "local");
    ///
    /// config.add_repository(repo);
    /// ```
    pub fn add_repository(&mut self, repo: Repository) {
        self.repositories.push(repo);
    }

    /// Returns a reference on the vector containing all the mirrors.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate libnest;
    ///
    /// use libnest::config::Config;
    /// use libnest::repository::Repository;
    ///
    /// let mut config = Config::new();
    /// let repo = Repository::new(&config, "local");
    ///
    /// assert_eq!(config.repositories().len(), 0);
    /// config.add_repository(repo);
    /// assert_eq!(config.repositories().len(), 1);
    /// ```
    pub fn repositories(&self) -> Iter<Repository> {
        self.repositories.iter()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config::new()
    }
}

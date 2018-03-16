//! Nest configuration parsing and handle.

use std::path::{Path, PathBuf};

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
    /// extern crate libnest;
    ///
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
    #[inline]
    pub fn cache(&self) -> &Path {
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
    /// config.repositories_mut().push(repo);
    /// ```
    #[inline]
    pub fn push_repository(&mut self, repo: Repository) {
        self.repositories.push(repo);
    }

    /// Yields a reference to the underlying `Vec<Repository>`
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
    /// extern crate libnest;
    ///
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

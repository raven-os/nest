//! Repositories and mirrors

use std::path::{Path, PathBuf};

use config::Config;

/// A repository.
///
/// It's made of a name and a list of mirrors.
///
/// # Examples
///
/// ```
/// extern crate libnest;
///
/// use libnest::config::Config;
/// use libnest::repository::{Repository, Mirror};
///
/// // We are going to need some configuration
/// let config = Config::new();
///
/// // First, create an empty repository with name "test":
/// let mut repo = Repository::new(&config, "test");
/// assert!(repo.mirrors().is_empty());
///
/// // Then, let's add a mirror:
/// repo.add_mirror(Mirror::new("http://example.com"));
/// assert_eq!(repo.mirrors().len(), 1);
///
/// // We can now iterate through all of them:
/// for mirror in repo.mirrors() {
///     println!("{}: {}", repo.name(), mirror.url());
/// }
/// ```
#[derive(Debug)]
pub struct Repository {
    name: String,
    mirrors: Vec<Mirror>,
    cache: Cache,
}

impl Repository {
    /// Creates a new, empty `Repository` with the given name.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate libnest;
    ///
    /// use libnest::config::Config;
    /// use libnest::repository::Repository;
    ///
    /// let config = Config::new();
    /// let repo = Repository::new(&config, "test");
    /// ```
    pub fn new(config: &Config, name: &str) -> Repository {
        let mut cache = config.cache().clone();
        cache.push(name);
        let cache = Cache::new(cache);
        Repository {
            name: String::from(name),
            mirrors: Vec::new(),
            cache,
        }
    }

    /// Returns the name of the repository.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate libnest;
    ///
    /// use libnest::config::Config;
    /// use libnest::repository::Repository;
    ///
    /// let config = Config::new();
    /// let repo = Repository::new(&config, "test");
    /// assert_eq!(repo.name(), "test");
    /// ```
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns a reference over a `Vec<Mirror>` representing all the mirrors of the repository.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate libnest;
    ///
    /// use libnest::config::Config;
    /// use libnest::repository::Repository;
    ///
    /// let config = Config::new();
    /// let repo = Repository::new(&config, "test");
    ///
    /// assert_eq!(repo.mirrors().len(), 0);
    /// ```
    pub fn mirrors(&self) -> &Vec<Mirror> {
        &self.mirrors
    }

    /// Returns a `Cache` representing the locale cache for the repository.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate libnest;
    ///
    /// use std::path::Path;
    ///
    /// use libnest::config::Config;
    /// use libnest::repository::Repository;
    ///
    /// let config = Config::new();
    /// let repo = Repository::new(&config, "test");
    /// let cache = repo.cache();
    ///
    /// assert_eq!(cache.path(), config.cache().join("test"));
    /// ```
    pub fn cache(&self) -> &Cache {
        &self.cache
    }

    /// Adds a mirror to the end of the mirrors list, meaning it has the lowest priority.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate libnest;
    ///
    /// use libnest::repository::{Repository, Mirror};
    /// use libnest::config::Config;
    ///
    /// let config = Config::new();
    /// let mut repo = Repository::new(&config, "test");
    ///
    /// repo.add_mirror(Mirror::new("http://example.com"));
    /// assert_eq!(repo.mirrors().len(), 1);
    /// ```
    pub fn add_mirror(&mut self, mirror: Mirror) {
        self.mirrors.push(mirror);
    }
}

/// A mirror for a given repository.
///
/// It's basically a wrapper arround an URL.
///
/// # Examples
///
/// ```
/// extern crate libnest;
///
/// use libnest::repository::Mirror;
///
/// let mirror = Mirror::new("http://example.com");
///
/// println!("Mirror's url: {}", mirror.url());
/// ```
#[derive(Debug)]
pub struct Mirror {
    url: String,
}

impl Mirror {
    /// Creates a new mirror from an url.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate libnest;
    ///
    /// use libnest::repository::Mirror;
    ///
    /// let m = Mirror::new("http://stable.raven-os.org/");
    /// ```
    pub fn new(url: &str) -> Mirror {
        Mirror {
            url: url.to_string(),
        }
    }

    /// Returns the mirror's url.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate libnest;
    ///
    /// use libnest::repository::Mirror;
    ///
    /// let m = Mirror::new("http://stable.raven-os.org/");
    ///
    /// println!("Stable URL: {}", m.url());
    /// ```
    pub fn url(&self) -> &str {
        &self.url
    }
}

/// The cache of a repository on the filesystem.
///
/// This cache holds metadatas about the repository, most notably a list of it's packages
/// and their name, versions, description, dependencies etc.
///
/// This structure is used to browse this cache and retrieve any kind of informations.
#[derive(Debug)]
pub struct Cache {
    path: PathBuf,
}

impl Cache {
    /// Creates (or loads) a new cache located at the given path
    pub(crate) fn new(path: PathBuf) -> Cache {
        Cache { path }
    }

    /// Return the path of the cache.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate libnest;
    ///
    /// use std::path::Path;
    ///
    /// use libnest::config::Config;
    /// use libnest::repository::Repository;
    ///
    /// let config = Config::new();
    /// let repo = Repository::new(&config, "test");
    /// let cache = repo.cache();
    ///
    /// assert_eq!(cache.path(), config.cache().join("test"));
    /// ```
    pub fn path(&self) -> &Path {
        &self.path
    }
}

//! Repositories and mirrors

use std::path::{Path, PathBuf};

use config::NestConfig;

/// A repository.
///
/// It's made of a name and a list of mirrors.
///
/// # Examples
///
/// ```
/// extern crate libnest;
///
/// use libnest::config::NestConfig;
/// use libnest::repository::{Repository, Mirror};
///
/// // We are going to need some configuration
/// let config = NestConfig::new();
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
    cache: PathBuf,
}

impl Repository {
    /// Creates a new, empty `Repository` with the given name.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate libnest;
    ///
    /// use libnest::config::NestConfig;
    /// use libnest::repository::Repository;
    ///
    /// let config = NestConfig::new();
    /// let repo = Repository::new(&config, "test");
    /// ```
    pub fn new(config: &NestConfig, name: &str) -> Repository {
        let mut cache = config.cache().clone();
        cache.push(name);
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
    /// use libnest::config::NestConfig;
    /// use libnest::repository::Repository;
    ///
    /// let config = NestConfig::new();
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
    /// use libnest::config::NestConfig;
    /// use libnest::repository::Repository;
    ///
    /// let config = NestConfig::new();
    /// let repo = Repository::new(&config, "test");
    ///
    /// assert_eq!(repo.mirrors().len(), 0);
    /// ```
    pub fn mirrors(&self) -> &Vec<Mirror> {
        &self.mirrors
    }

    /// Returns a `Path` to the cache of the repository.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate libnest;
    ///
    /// use std::path::Path;
    /// use libnest::repository::Repository;
    /// use libnest::config::NestConfig;
    ///
    /// let config = NestConfig::new();
    /// let repo = Repository::new(&config, "test");
    ///
    /// assert_eq!(repo.cache(), Path::new("/var/lib/nest/cache/test"));
    /// ```
    pub fn cache(&self) -> &Path {
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
    /// use libnest::config::NestConfig;
    ///
    /// let config = NestConfig::new();
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

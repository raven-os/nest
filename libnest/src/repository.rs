//! Repositories and mirrors
//!
//! This module contains handle to repositories and repository-related stuff, like mirrors.
//!
//! It lets you create new repositories, fill them with mirrors and interact with them.

use std::str;
use std::error;
use std::convert::TryFrom;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::fmt::{self, Display, Formatter};
use std::fs::{self, File};

use json;
use curl::easy::Easy;

use config::Config;
use package::Package;

/// A wrapper on the errors the `pull` operation can produce.
pub type PullRepositoryError = Box<error::Error>;

/// A repository.
///
/// It's made of a name and a list of mirrors.
///
/// # Examples
///
/// ```
/// # extern crate libnest;
/// use libnest::config::Config;
/// use libnest::repository::{Repository, Mirror};
///
/// // We are going to need some configuration
/// let config = Config::new();
///
/// // First, create an empty repository with name "test":
/// let mut repo = Repository::new(&config, "test");
///
/// // Then, let's add a mirror:
/// {
///     let mirrors = repo.mirrors_mut();
///     assert!(mirrors.is_empty());
///
///     mirrors.push(Mirror::new("http://example.com"));
///     assert_eq!(mirrors.len(), 1);
/// }
///
/// // We can now iterate through all of them:
/// for mirror in repo.mirrors() {
///     println!("{}: {}", repo.name(), mirror.url());
/// }
/// ```
#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
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
    /// # extern crate libnest;
    /// use libnest::config::Config;
    /// use libnest::repository::Repository;
    ///
    /// let config = Config::new();
    /// let repo = Repository::new(&config, "test");
    /// ```
    #[inline]
    pub fn new(config: &Config, name: &str) -> Repository {
        let mut cache = config.cache().to_path_buf();
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
    /// # extern crate libnest;
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
    /// # extern crate libnest;
    /// use libnest::config::Config;
    /// use libnest::repository::Repository;
    ///
    /// let config = Config::new();
    /// let repo = Repository::new(&config, "test");
    ///
    /// let mirrors = repo.mirrors();
    ///
    /// assert_eq!(mirrors.len(), 0);
    /// ```
    #[inline]
    pub fn mirrors(&self) -> &Vec<Mirror> {
        &self.mirrors
    }

    /// Returns a mutable reference over a `Vec<Mirror>` representing all the mirrors of the repository.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
    /// use libnest::config::Config;
    /// use libnest::repository::{Repository, Mirror};
    ///
    /// let config = Config::new();
    /// let mut repo = Repository::new(&config, "test");
    ///
    /// let mirrors = repo.mirrors_mut();
    /// assert!(mirrors.is_empty());
    ///
    /// mirrors.push(Mirror::new("http://example.com"));
    /// assert_eq!(mirrors.len(), 1);
    /// ```
    #[inline]
    pub fn mirrors_mut(&mut self) -> &mut Vec<Mirror> {
        &mut self.mirrors
    }

    /// Returns a `Cache` representing the local cache for the repository.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
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
    #[inline]
    pub fn cache(&self) -> &Cache {
        &self.cache
    }

    /// Pulls the repository with the given mirror, analyzes the result and updates local cache.
    ///
    /// # Blocking function
    ///
    /// This is a blocking function, that's why the `cb` parameter is a closure that let's you
    /// update any kind of progress bar during the download.
    ///
    /// The first parameter is the number of downloaded bytes, and the second one is the total
    /// number of bytes. The closure must return a bool: `true` to continue, `false` to interrupt
    /// the download.
    ///
    /// # Filesystem
    ///
    /// This operation assumes the current user has the rights to write in the local cache.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate libnest;
    /// use libnest::config::Config;
    /// use libnest::repository::{Repository, Mirror};
    ///
    /// // Let's setup a basic configuration
    /// let mut config = Config::new();
    /// let mut repo = Repository::new(&config, "stable");
    /// let mirror = Mirror::new("http://example.com");
    ///
    /// repo.mirrors_mut().push(mirror);
    /// config.repositories_mut().push(repo);
    ///
    /// // Pull all repositories
    /// for repo in config.repositories() {
    ///     // Pull all mirrors
    ///     for mirror in repo.mirrors() {
    ///         let r = repo.pull(&config, &mirror, |cur: f64, max: f64| {
    ///             println!("Progress: {}/{}", cur, max);
    ///             true
    ///         });
    ///
    ///         // Analyze result
    ///         match r {
    ///             Ok(_) => {
    ///                 println!("{} pulled correctly", repo.name());
    ///                 break; // Don't pull other mirrors in case of success
    ///             }
    ///             Err(e) => {
    ///                 eprintln!("Couldn't pull {}: {}", repo.name(), e);
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    pub fn pull<F>(
        &self,
        config: &Config,
        mirror: &Mirror,
        mut cb: F,
    ) -> Result<(), PullRepositoryError>
    where
        F: FnMut(f64, f64) -> bool,
    {
        let mut data = Vec::new();
        let mut handle = Easy::try_from(config)?;
        let pull_url = mirror.url().to_string() + "/pull";

        // Download data from mirror
        handle.url(&pull_url)?;
        handle.progress(true)?;
        {
            let mut transfer = handle.transfer();

            transfer.write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })?;
            transfer.progress_function(|a: f64, b: f64, _: f64, _: f64| cb(b, a))?;
            transfer.perform()?;
        }

        // Parse data to UTF8 and deserialize it
        let utf8_data = str::from_utf8(&data)?;
        let packages: Vec<Package> = json::from_str(utf8_data)?;

        // Write output to disk
        for package in packages {
            self.cache.update_package(&package)?;
        }
        Ok(())
    }
}

impl Display for Repository {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// A mirror for a given repository.
///
/// It's basically a wrapper arround an URL.
///
/// # Examples
///
/// ```
/// # extern crate libnest;
/// use libnest::repository::Mirror;
///
/// let mirror = Mirror::new("http://example.com");
///
/// println!("Mirror's url: {}", mirror.url());
/// ```
#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Mirror {
    url: String,
}

impl Mirror {
    /// Creates a new mirror from an url.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
    /// use libnest::repository::Mirror;
    ///
    /// let m = Mirror::new("http://stable.raven-os.org/");
    /// ```
    #[inline]
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
    /// # extern crate libnest;
    /// use libnest::repository::Mirror;
    ///
    /// let m = Mirror::new("http://stable.raven-os.org/");
    ///
    /// println!("Stable URL: {}", m.url());
    /// ```
    #[inline]
    pub fn url(&self) -> &str {
        &self.url
    }
}

impl Display for Mirror {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.url)
    }
}

/// The cache of a repository on the filesystem.
///
/// This cache holds metadatas about the repository, most notably a list of it's packages
/// and their name, versions, description, dependencies etc.
///
/// This structure is used to browse this cache and retrieve any kind of informations.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Cache {
    path: PathBuf,
}

impl Cache {
    /// Creates (or loads) a new cache located at the given path
    #[inline]
    pub(crate) fn new(path: PathBuf) -> Cache {
        Cache { path }
    }

    /// Return the path of the cache.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
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
    #[inline]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Updates the cache with the given metadatas.
    ///
    /// # Filesystem
    ///
    /// This operation assumes the current user has the rights to write in the local cache.
    pub(crate) fn update_package(&self, package: &Package) -> Result<PathBuf, io::Error> {
        let mut path = self.path.clone();
        let json = json::to_string(package)?;

        // Create category folder
        path.push(package.category());
        fs::create_dir_all(&path)?;

        // Create package file
        path.push(package.name());
        let mut file = File::create(&path)?;

        // Write content
        file.write_all(json.as_bytes())?;
        file.write_all(&[b'\n'])?;
        Ok(path)
    }
}

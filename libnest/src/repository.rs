//! Repositories and mirrors
//!
//! This module contains handle to repositories and repository-related stuff, like mirrors.
//!
//! It lets you create new repositories, fill them with mirrors and interact with them.

use std::convert::TryFrom;
use std::error;
use std::fmt::{self, Display, Formatter};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::str;

use curl::easy::Easy;
use json;

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
    cache: RepositoryCache,
}

impl Repository {
    /// Creates a new, empty [`Repository`](struct.Repository.html) with the given name.
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

        Repository {
            name: String::from(name),
            mirrors: Vec::new(),
            cache: RepositoryCache::new(cache),
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

    /// Returns a reference over a [`Vec<Mirror>`](https://doc.rust-lang.org/std/vec/struct.Vec.html)
    /// representing all the mirrors of the repository.
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

    /// Returns a mutable reference over a [`Vec<Mirror>`](https://doc.rust-lang.org/std/vec/struct.Vec.html) representing all the mirrors of the repository.
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

    /// Returns a structure representing the local cache of the repository.
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
    pub fn cache(&self) -> &RepositoryCache {
        &self.cache
    }

    /// Pulls the repository with the given [`Mirror`](struct.Mirror.html), analyzes the result and updates local cache.
    ///
    /// # Blocking function
    ///
    /// This is a blocking function, that's why the `cb` parameter is a
    /// [`FnMut(f64, f64) -> bool`](https://doc.rust-lang.org/std/ops/trait.FnMut.html) that let's you
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

        // Remove existing cache
        if self.cache.path().exists() {
            fs::remove_dir_all(self.cache.path())?;
        }
        fs::create_dir_all(self.cache.path())?;

        // Write output to disk
        for package in packages {
            self.cache.update(&package)?;
        }
        Ok(())
    }

    /// Downloads a package from the repository using the given mirror.
    ///
    /// # Blocking function
    ///
    /// This is a blocking function, that's why the `cb` parameter is a
    /// [`FnMut(f64, f64) -> bool`](https://doc.rust-lang.org/std/ops/trait.FnMut.html) that let's you
    /// update any kind of progress bar during the download.
    ///
    /// The first parameter is the number of downloaded bytes, and the second one is the total
    /// number of bytes. The closure must return a bool: `true` to continue, `false` to interrupt
    /// the download.
    ///
    /// # Filesystem
    ///
    /// This operation assumes the current user has the rights to read the local cache and write in the
    /// download directory (see [`config::download_path`](../config/struct.Config.html#method.download_path)).
    ///
    /// # Examples
    ///
    /// This exemple uses [`CacheQuery`](../query/struct.CacheQuery.html).
    ///
    /// ```no_run
    /// # extern crate libnest;
    /// # use std::io;
    /// #
    /// # fn download() -> Result<(), io::Error> {
    /// use libnest::config::Config;
    /// use libnest::repository::{Repository, Mirror};
    /// use libnest::query::CacheQuery;
    ///
    /// // Let's setup a basic configuration
    /// let mut config = Config::new();
    /// let mut repo = Repository::new(&config, "stable");
    /// let mirror = Mirror::new("http://example.com");
    ///
    /// repo.mirrors_mut().push(mirror);
    /// config.repositories_mut().push(repo);
    ///
    /// // We are going to download `stable::shell/dash`. Let's look for it.
    /// let mut query = CacheQuery::new(&config);
    /// query.with_repository("stable");
    /// query.with_category("shell");
    /// query.with_name("dash");
    ///
    /// if let Some(target) = query.perform()?.get(0) {
    ///     let repo = target.repository();
    ///
    ///     // Try all mirrors
    ///     for mirror in repo.mirrors() {
    ///         let r = repo.download(&config, &mirror, target.content(), |cur: f64, max: f64| {
    ///             println!("Progress: {}/{}", cur, max);
    ///             true
    ///         });
    ///
    ///         // Analyze result
    ///         match r {
    ///             Ok(_) => {
    ///                 println!("{} pulled correctly", repo.name());
    ///                 break; // Don't try other mirrors in case of success
    ///             }
    ///             Err(e) => {
    ///                 eprintln!("Couldn't pull {}: {}", repo.name(), e);
    ///             }
    ///         }
    ///     }
    /// } else {
    ///     eprintln!("Couldn't find the package");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn download<F>(
        &self,
        config: &Config,
        mirror: &Mirror,
        package: &Package,
        mut cb: F,
    ) -> Result<(), PullRepositoryError>
    where
        F: FnMut(f64, f64) -> bool,
    {
        // Create download directory
        let mut path = config.download_path().to_path_buf();
        path.push(package.category());
        fs::create_dir_all(path.clone())?;

        // Create destination path
        path.push(package.name());
        path.set_extension("tar");

        // Init download
        let mut file = File::create(path)?;
        let mut handle = Easy::try_from(config)?;
        let pull_url =
            mirror.url().to_string() + "/download/" + package.category() + "/" + package.name();

        // Download data from mirror
        handle.url(&pull_url)?;
        handle.progress(true)?;
        {
            let mut transfer = handle.transfer();

            transfer.write_function(|new_data| match file.write_all(new_data) {
                Ok(_) => Ok(new_data.len()),
                Err(_) => Ok(0),
            })?;
            transfer.progress_function(|a: f64, b: f64, _: f64, _: f64| cb(b, a))?;
            transfer.perform()?;
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
/// This cache holds a bunch of cache for each categories, which contains list of packages
/// and their name, versions, description, dependencies etc.
///
/// This structure is used to browse this cache and retrieve any kind of informations.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct RepositoryCache {
    path: PathBuf,
}

impl RepositoryCache {
    /// Creates (or loads) a new cache located at the given path
    #[inline]
    pub(crate) fn new(path: PathBuf) -> RepositoryCache {
        RepositoryCache { path }
    }

    /// Return the path of the repository cache.
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

    /// Returns the [`CategoryCache`](struct.CategoryCache.html) of a given category, or `None` if it was not found.
    ///
    /// # Filesystem
    ///
    /// This operation assumes the current user has the rights to read the local cache.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate libnest;
    ///
    /// use libnest::repository::Repository;
    /// use libnest::config::Config;
    ///
    /// let config = Config::new();
    /// let repository = Repository::new(&config, "stable");
    ///
    /// // Retrieves the cache of the `shell` category for the `stable` repository.
    /// assert!(repository.cache().category("shell").is_some());
    /// ```
    #[inline]
    pub fn category(&self, category: &str) -> Option<CategoryCache> {
        let mut path = self.path.clone();

        // Look for category folder
        path.push(category);
        if path.exists() && path.is_dir() {
            Some(CategoryCache::new(path))
        } else {
            None
        }
    }

    /// Returns an iterator over all [`CategoryCache`](struct.CategoryCache.html) this cache contains (with their
    /// corresponding name), or an
    /// [`std::io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html) in case of failure.
    ///
    /// # Filesystem
    ///
    /// This operation assumes the current user has the rights to read the local cache.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate libnest;
    /// #
    /// # use std::io;
    /// #
    /// # fn test() -> Result<(), io::Error> {
    /// use libnest::repository::Repository;
    /// use libnest::config::Config;
    ///
    /// let config = Config::new();
    /// let repository = Repository::new(&config, "stable");
    ///
    /// // Retrieves the cache of the `shell` category for the `stable` repository.
    /// for (name, _category_cache) in repository.cache().categories()? {
    ///     println!("Found category {}", name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn categories(&self) -> Result<impl Iterator<Item = (String, CategoryCache)>, io::Error> {
        let mut vec = Vec::new();

        for category_path in fs::read_dir(&self.path)? {
            let dir = category_path?;
            if let Ok(name) = dir.file_name().into_string() {
                vec.push((name, CategoryCache::new(dir.path())));
            }
        }
        Ok(vec.into_iter())
    }

    /// Updates the cache of the given package
    #[inline]
    pub(crate) fn update(&self, package: &Package) -> Result<PackageCache, io::Error> {
        let mut path = self.path.clone();

        // Create category folder
        path.push(package.category());
        fs::create_dir_all(path.clone())?;

        // Update package
        CategoryCache::new(path).update(package)
    }
}

/// The cache of a category for a given repository on the filesystem.
///
/// This cache holds a list of packages and their name, versions, description, dependencies etc.
///
/// This structure is used to browse this cache and retrieve any kind of informations.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct CategoryCache {
    path: PathBuf,
}

impl CategoryCache {
    #[inline]
    pub(crate) fn new(path: PathBuf) -> CategoryCache {
        CategoryCache { path }
    }

    /// Same as [`RepositoryCache::path`](struct.repositorycache.html#method.path), but returns the path
    /// of the category's cache instead.
    #[inline]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Same as [`RepositoryCache::category`](struct.RepositoryCache.html#method.category), but returns the
    /// [`PackageCache`](struct.PackageCache.html) of the given package instead.
    #[inline]
    pub fn package(&self, name: &str) -> Option<Result<PackageCache, io::Error>> {
        let mut path = self.path.clone();

        path.push(name);
        if path.exists() {
            Some(PackageCache::load(path))
        } else {
            None
        }
    }

    /// Same as [`RepositoryCache::category`](struct.RepositoryCache.html#method.category), but returns an iterator
    /// over all [`PackageCache`](struct.PackageCache.html) this cache contains instead.
    #[inline]
    pub fn packages(&self) -> Result<impl Iterator<Item = PackageCache>, io::Error> {
        let mut vec = Vec::new();

        for package_path in fs::read_dir(&self.path)? {
            vec.push(PackageCache::load(package_path?.path())?);
        }
        Ok(vec.into_iter())
    }

    /// Updates the cache of the given package
    pub(crate) fn update(&self, package: &Package) -> Result<PackageCache, io::Error> {
        let mut path = self.path.clone();
        path.push(package.name());

        // Write content
        let mut file = File::create(&path)?;
        file.write_all(json::to_string(package)?.as_bytes())?;
        file.write_all(&[b'\n'])?;
        PackageCache::load(path)
    }
}

/// The cache for a package.
///
/// Basically, this is a wrapper around a package and a path.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct PackageCache {
    path: PathBuf,
    content: Package,
}

impl PackageCache {
    #[inline]
    pub(crate) fn load(path: PathBuf) -> Result<PackageCache, io::Error> {
        let file = File::open(path.clone())?;

        Ok(PackageCache {
            path,
            content: json::from_reader(file)?,
        })
    }

    /// Same as [`RepositoryCache::path`](struct.RepositoryCache.html#method.path), but returns the path of the package's cache instead.
    #[inline]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns the package's metadatas
    #[inline]
    pub fn content(&self) -> &Package {
        &self.content
    }
}

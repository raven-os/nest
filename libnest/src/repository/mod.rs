//! Repositories and mirrors
//!
//! This module contains types to handle repositories and repository-related stuff, like mirrors.
//!
//! It lets you create new repositories, fill them with mirrors and interact with them.

mod cache;
mod mirror;

use std::convert::TryFrom;
use std::fmt::{self, Display, Formatter};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::str;

use curl::{self, easy::Easy};
use json;

use config::Config;
use error::*;
use package::Manifest;

use failure::{Error, ResultExt};

pub use self::cache::{CategoryCache, ManifestCache, RepositoryCache};
pub use self::mirror::Mirror;

/// A repository.
///
/// It's made of a name and a list of mirrors.
///
/// # Examples
///
/// ```
/// # extern crate libnest;
/// extern crate url;
///
/// use url::Url;
/// use libnest::repository::{Repository, Mirror};
/// # use std::error::Error;
/// # fn test() -> Result<(), Box<Error>> {
///
/// // First, create an empty repository with name "stable":
/// let mut repo = Repository::new("stable");
///
/// // Then, let's add a mirror:
/// {
///     let mirrors = repo.mirrors_mut();
///     assert!(mirrors.is_empty());
///
///     mirrors.push(Mirror::new(Url::parse("http://stable.raven-os.org")?));
///     assert_eq!(mirrors.len(), 1);
/// }
///
/// // We can now iterate through all of them:
/// for mirror in repo.mirrors() {
///     println!("{}: {}", repo.name(), mirror.url());
/// }
/// # Ok(())
/// # }
/// # fn main() {
/// #   test().unwrap();
/// # }
/// ```
#[derive(Clone, Eq, PartialEq, Hash, Debug, Default, Serialize, Deserialize)]
pub struct Repository {
    name: String,
    mirrors: Vec<Mirror>,
}

impl Repository {
    /// Creates a new, empty repository with the given name.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
    /// use libnest::repository::Repository;
    ///
    /// let repo = Repository::new("stable");
    /// ```
    #[inline]
    pub fn new(name: &str) -> Repository {
        Repository {
            name: String::from(name),
            mirrors: Vec::new(),
        }
    }

    /// Returns the name of the repository.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
    /// use libnest::repository::Repository;
    ///
    /// let repo = Repository::new("stable");
    ///
    /// assert_eq!(repo.name(), "stable");
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
    /// use libnest::repository::Repository;
    ///
    /// let repo = Repository::new("stable");
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
    /// extern crate url;
    ///
    /// use url::Url;
    /// use libnest::repository::{Repository, Mirror};
    /// # use std::error::Error;
    /// # fn test() -> Result<(), Box<Error>> {
    ///
    /// let mut repo = Repository::new("stable");
    ///
    /// let mirrors = repo.mirrors_mut();
    /// assert!(mirrors.is_empty());
    ///
    /// mirrors.push(Mirror::new(Url::parse("http://stable.raven-os.org")?));
    /// assert_eq!(mirrors.len(), 1);
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #   test().unwrap();
    /// # }
    /// ```
    #[inline]
    pub fn mirrors_mut(&mut self) -> &mut Vec<Mirror> {
        &mut self.mirrors
    }

    /// Returns a [`RepositoryCache`](cache/struct.RepositoryCache.html) representing the local cache of the repository.
    ///
    /// The location of the cache is provided by the given configuration.
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
    /// let repo = Repository::new("stable");
    /// let cache = repo.cache(&config);
    ///
    /// assert_eq!(cache.path(), Path::new("/var/nest/cache/stable"));
    /// ```
    #[inline]
    pub fn cache(&self, config: &Config) -> RepositoryCache {
        let mut path = config.cache().to_path_buf();
        path.push(self.name());
        RepositoryCache::new(path)
    }

    /// Pulls the repository with the given [`Mirror`](mirror/struct.Mirror.html), analyzes the result and updates local cache.
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
    /// # extern crate url;
    /// # use std::error::Error;
    /// use url::Url;
    /// use libnest::config::Config;
    /// use libnest::repository::{Repository, Mirror};
    ///
    /// # fn test() -> Result<(), Box<Error>> {
    /// // Let's setup a basic configuration
    /// let mut config = Config::new();
    /// let mut repo = Repository::new("stable");
    /// let mirror = Mirror::new(Url::parse("http://stable.raven-os.org")?);
    ///
    /// repo.mirrors_mut().push(mirror);
    /// config.repositories_mut().push(repo);
    ///
    /// // Pull all repositories
    /// for repo in config.repositories() {
    ///     // Pull all mirrors
    ///     for mirror in repo.mirrors() {
    ///         let res = repo.pull(&config, &mirror, |cur: f64, max: f64| {
    ///             println!("Progress: {}/{}", cur, max);
    ///             true
    ///         });
    ///
    ///         // Analyze result
    ///         match res {
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
    /// # Ok(())
    /// # }
    /// ```
    pub fn pull<F>(&self, config: &Config, mirror: &Mirror, mut cb: F) -> Result<(), Error>
    where
        F: FnMut(f64, f64) -> bool,
    {
        let mut data = Vec::new();

        // Download data from mirror and catch all CURL errors
        let pull_url = mirror.url().join("pull")?;

        let r: Result<_, curl::Error> = do catch {
            let mut handle = Easy::try_from(config)?;
            handle.url(pull_url.as_str())?;
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
            ()
        };
        r.map_err(|e| {
            use std::error::Error;
            PullErrorKind::Download(e.description().to_string())
        }).context(pull_url.to_string())?;

        // Parse data to UTF8 and deserialize it.
        let r: Result<Vec<Manifest>, Error> = do catch {
            let utf8_data = str::from_utf8(&data)?;
            json::from_str(utf8_data)?
        };
        let manifests = r.or_else(|_| Err(PullErrorKind::InvalidData(pull_url.clone())))?;

        // Remove existing cache
        let cache = self.cache(config);
        let display_cache = cache.path().display().to_string();
        if cache.path().exists() {
            fs::remove_dir_all(cache.path())
                .context(PullErrorKind::CantRemoveCache(display_cache.clone()))?;
        }
        fs::create_dir_all(cache.path()).context(PullErrorKind::CantCreateCache(display_cache))?;

        // Write output to disk
        for manifest in manifests {
            cache.update(&manifest)?;
        }
        Ok(())
    }

    /// Downloads a package's data from the repository using the given mirror.
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
    /// This exemple uses [`CacheQuery`](../query/struct.CacheQuery.html).
    ///
    /// ```no_run
    /// # extern crate libnest;
    /// # extern crate url;
    /// # use std::error::Error;
    /// #
    /// # fn download() -> Result<(), Box<Error>> {
    /// use url::Url;
    /// use libnest::config::Config;
    /// use libnest::repository::{Repository, Mirror};
    /// use libnest::query::CacheQuery;
    ///
    /// // Let's setup a basic configuration
    /// let mut config = Config::new();
    /// let mut repo = Repository::new("stable");
    /// let mirror = Mirror::new(Url::parse("http://stable.raven-os.org")?);
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
    ///         let res = repo.download(&config, &mirror, target.manifest(), &target.data_path(&config), |cur: f64, max: f64| {
    ///             println!("Progress: {}/{}", cur, max);
    ///             true
    ///         });
    ///
    ///         // Analyze result
    ///         match res {
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
        manifest: &Manifest,
        dest: &Path,
        mut cb: F,
    ) -> Result<(), Error>
    where
        F: FnMut(f64, f64) -> bool,
    {
        // Init download
        let mut file = File::create(dest).context(dest.display().to_string())?;
        let dl_url = mirror.url().join(&format!(
            "/download/{}/{}",
            manifest.metadata().category(),
            manifest.metadata().name(),
        ))?;

        // Download data from mirror
        let r: Result<_, curl::Error> = do catch {
            let mut handle = Easy::try_from(config)?;
            handle.url(dl_url.as_str())?;
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
            ()
        };
        r.map_err(|e| {
            use std::error::Error;
            DownloadErrorKind::Download(e.description().to_string())
        }).context(dl_url.to_string())?;

        Ok(())
    }
}

impl Display for Repository {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

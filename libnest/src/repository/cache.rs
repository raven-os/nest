//! Types to represent the cache of a repository on local disk.
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use failure::{Error, ResultExt};
use toml;

use error::CacheErrorKind;
use package::Manifest;

/// The cache of a repository on the filesystem.
///
/// This cache holds a bunch of cache for each categories, which contains a list of manifests
/// and their name, versions, description, dependencies, etc.
///
/// This structure is used to browse this cache and retrieve any kind of informations.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct RepositoryCache {
    path: PathBuf,
}

impl RepositoryCache {
    /// Creates (or loads) a new cache located at the given path.
    #[inline]
    pub(crate) fn new(path: PathBuf) -> RepositoryCache {
        RepositoryCache { path }
    }

    /// Returns the path of the repository's cache.
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
    /// use libnest::config::Config;
    /// use libnest::repository::Repository;
    ///
    /// let config = Config::new();
    ///
    /// let repository_cache = Repository::new("stable").cache(&config);
    /// match repository_cache.category("shell") {
    ///     Some(category) => println!("The category exists !"),
    ///     None => println!("The category \"shell\" doesn't exist")
    /// }
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
    /// use libnest::repository::Repository;
    /// use libnest::config::Config;
    ///
    /// let config = Config::new();
    ///
    /// let repository = Repository::new("stable");
    /// let repository_cache = repository.cache(&config);
    /// match repository_cache.categories() {
    ///     Ok(categories) => println!("There are {} categories for this repository", categories.count()),
    ///     Err(e) => eprintln!("Couldn't access the categories of this repository: {}", e)
    /// }
    /// ```
    #[inline]
    pub fn categories(&self) -> Result<impl Iterator<Item = (String, CategoryCache)>, Error> {
        let mut vec = Vec::new();

        let r: Result<_, Error> = do catch {
            if self.path.exists() {
                for category_path in fs::read_dir(&self.path)? {
                    let dir = category_path?;
                    if let Ok(name) = dir.file_name().into_string() {
                        vec.push((name, CategoryCache::new(dir.path())));
                    }
                }
            }
            ()
        };
        r.context(CacheErrorKind::IO(self.path.display().to_string()))?;
        Ok(vec.into_iter())
    }

    /// Updates the cache of the given manifest.
    ///
    /// # Filesystem
    ///
    /// This operation assumes the current user has the rights to write the local cache.
    #[inline]
    pub(crate) fn update(&self, manifest: &Manifest) -> Result<ManifestCache, Error> {
        let mut path = self.path.clone();

        // Create category folder
        path.push(manifest.metadata().category());
        fs::create_dir_all(path.clone()).context(CacheErrorKind::IO(path.display().to_string()))?;

        // Update manifest
        CategoryCache::new(path).update(manifest)
    }
}

/// The cache of a category for a given repository on the filesystem.
///
/// This cache holds a list of manifests: package's name, versions, description, dependencies, etc.
///
/// This structure is used to browse this cache and retrieve any kind of information.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct CategoryCache {
    path: PathBuf,
}

impl CategoryCache {
    /// Creates a new category cache located at the given path.
    #[inline]
    pub(crate) fn new(path: PathBuf) -> CategoryCache {
        CategoryCache { path }
    }

    /// Returns the path of the category's cache.
    #[inline]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns the [`ManifestCache`](struct.ManifestCache.html) of a given manifest, or `None` if it was not found.
    ///
    /// # Filesystem
    ///
    /// This operation assumes the current user has the rights to read the local cache.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate libnest;
    /// use libnest::config::Config;
    /// use libnest::repository::Repository;
    ///
    /// let config = Config::new();
    ///
    /// let repository = Repository::new("stable");
    /// let manifest = repository.cache(&config).category("shell")
    ///     .and_then(|category| category.manifest("bash"));
    ///
    /// if let Some(manifest) = manifest {
    ///     println!("There is a package named bash within the shell category");
    /// } else {
    ///     println!("There is no such package");
    /// }
    /// ```
    #[inline]
    pub fn manifest(&self, name: &str) -> Option<Result<ManifestCache, Error>> {
        let mut path = self.path.clone();

        path.push(name);
        if path.exists() {
            Some(ManifestCache::load(path))
        } else {
            None
        }
    }

    /// Returns an iterator over all [`ManifestCache`](struct.ManifestCache.html) this cache contains, or an
    /// [`std::error::Error`](https://doc.rust-lang.org/std/error/trait.Error.html) in case of failure.
    ///
    /// # Filesystem
    ///
    /// This operation assumes the current user has the rights to read the local cache.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate libnest;
    /// use libnest::config::Config;
    /// use libnest::repository::Repository;
    ///
    /// let config = Config::new();
    ///
    /// let repository = Repository::new("stable");
    /// if let Some(category_cache) = repository.cache(&config).category("shell") {
    ///     match category_cache.manifests() {
    ///     Ok(manifests) => println!("There are {} manifests for this repository", manifests.count()),
    ///     Err(e) => eprintln!("Couldn't access the manifests of this repository: {}", e)
    ///     }
    /// } else {
    ///     println!("Can't find the category \"shell\" for the repository\"{}\"", repository.name());
    /// }
    /// ```
    #[inline]
    pub fn manifests(&self) -> Result<impl Iterator<Item = ManifestCache>, Error> {
        let mut vec = Vec::new();
        let context = CacheErrorKind::IO(self.path.display().to_string());

        for manifest_path in fs::read_dir(&self.path).context(context.clone())? {
            vec.push(ManifestCache::load(
                manifest_path.context(context.clone())?.path(),
            )?);
        }
        Ok(vec.into_iter())
    }

    /// Updates the cache of the given manifest.
    ///
    /// # Filesystem
    ///
    /// This operation assumes the current user has the rights to write the local cache.
    pub(crate) fn update(&self, manifest: &Manifest) -> Result<ManifestCache, Error> {
        let mut path = self.path.clone();
        path.push(manifest.metadata().name());
        let io_context = CacheErrorKind::IO(path.display().to_string());
        let serde_context = CacheErrorKind::Serialize(path.display().to_string());

        // Write content
        let mut file = File::create(&path).context(io_context.clone())?;
        file.write_all(
            toml::to_string_pretty(manifest)
                .context(serde_context.clone())?
                .as_bytes(),
        ).context(io_context.clone())?;
        file.write_all(&[b'\n']).context(io_context)?;
        ManifestCache::load(path)
    }
}

/// The cache of a manifest.
///
/// Basically, this is a wrapper around a manifest and a path.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct ManifestCache {
    path: PathBuf,
    manifest: Manifest,
}

impl ManifestCache {
    #[inline]
    /// Loads the cache of the manifest at the given path, or returns an
    /// [`std::error::Error`](https://doc.rust-lang.org/std/error/trait.Error.html) in case of failure.
    pub(crate) fn load(path: PathBuf) -> Result<ManifestCache, Error> {
        let display = path.display().to_string();
        let mut file = File::open(path.clone()).context(display.clone())?;
        let mut content = String::new();

        file.read_to_string(&mut content)
            .context(CacheErrorKind::IO(display.clone()))?;
        Ok(ManifestCache {
            path,
            manifest: toml::from_str(&content).context(CacheErrorKind::Deserialize(display))?,
        })
    }

    /// Returns the path of the manifest's cache.
    #[inline]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns the content of the cache.
    #[inline]
    pub fn manifest(&self) -> &Manifest {
        &self.manifest
    }
}

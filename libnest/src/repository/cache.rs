//! Types to represent the cache of a repository on local disk
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

    /// Updates the cache of the given manifest
    ///
    /// # Filesystem
    ///
    /// This operation assumes the current user has the rights to write the local cache.
    #[inline]
    pub(crate) fn update(&self, manifest: &Manifest) -> Result<ManifestCache, Error> {
        let mut path = self.path.clone();

        // Create category folder
        path.push(manifest.metadatas().category());
        fs::create_dir_all(path.clone()).context(CacheErrorKind::IO(path.display().to_string()))?;

        // Update manifest
        CategoryCache::new(path).update(manifest)
    }
}

/// The cache of a category for a given repository on the filesystem.
///
/// This cache holds a list of manifests: package's name, versions, description, dependencies etc.
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

    /// Updates the cache of the given manifest
    ///
    /// # Filesystem
    ///
    /// This operation assumes the current user has the rights to write the local cache.
    pub(crate) fn update(&self, manifest: &Manifest) -> Result<ManifestCache, Error> {
        let mut path = self.path.clone();
        path.push(manifest.metadatas().name());
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

    /// Returns the content of the cache
    #[inline]
    pub fn manifest(&self) -> &Manifest {
        &self.manifest
    }
}

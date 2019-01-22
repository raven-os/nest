//! Module to query and manipulate the cache of available packages
//! This cache is populated and updated by pull operations.

use super::errors::*;

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use failure::{Error, ResultExt};
use serde_json;

use crate::package::Package;
use crate::repository::Repository;

/// Structure representing the cache of available packages
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct AvailablePackages<'a> {
    cache_root: &'a Path,
}

impl<'a> AvailablePackages<'a> {
    #[allow(dead_code)] // TODO: Remove this when the function is used
    pub(crate) fn from(cache_root: &'a Path) -> Self {
        AvailablePackages { cache_root }
    }

    /// Erases the whole cache
    pub fn erase(&self) -> Result<(), Error> {
        if self.cache_root.exists() {
            fs::remove_dir_all(self.cache_root)
                .context(self.cache_root.display().to_string())
                .context(CacheErrorKind::CacheClearError)?;
        }
        Ok(())
    }

    /// Erases a given [`Repository`] from the cache
    pub fn erase_repository(&self, repository: &Repository) -> Result<(), Error> {
        let path = self.cache_root.join(repository.name());

        if path.exists() {
            fs::remove_dir_all(&path)
                .context(path.display().to_string())
                .context(CacheErrorKind::CacheClearError)?;
        }
        Ok(())
    }

    /// Creates or updates the cache entry for a given [`Package`]
    pub fn update(&self, package: &Package) -> Result<(), Error> {
        let metadata = package.manifest().metadata();

        let cache_path = self
            .cache_root
            .join(package.repository())
            .join(metadata.category())
            .join(metadata.name())
            .join(metadata.version().to_string());

        let res: Result<_, Error> = try {
            if let Some(parent) = cache_path.parent() {
                fs::create_dir_all(parent)?;
            }

            let mut file = File::create(&cache_path)?;
            file.write_all(serde_json::to_string_pretty(package.manifest())?.as_bytes())?;
            file.write_all(&[b'\n'])?;
        };
        res.context(cache_path.display().to_string())
            .context(CacheErrorKind::CacheWriteError)?;
        Ok(())
    }
}

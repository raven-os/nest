//! Module to query and manipulate the cache of available packages
//! This cache is populated and updated by pull operations.

mod query;

pub use self::query::{
    AvailablePackagesCacheQuery, AvailablePackagesCacheQueryStrategy, QueryResult,
};

use super::errors::*;

use std::fs::{self, File};
use std::io::Write;
use std::marker::PhantomData;
use std::path::Path;

use failure::{Error, ResultExt};
use serde_json;

use crate::lock_file::LockFileOwnership;
use crate::package::{PackageManifest, SoftPackageRequirement};
use crate::repository::Repository;

/// Structure representing the cache of available packages
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct AvailablePackages<'cache_root, 'lock_file> {
    cache_root: &'cache_root Path,
    phantom: PhantomData<&'lock_file LockFileOwnership>,
}

impl<'cache_root, 'lock_file> AvailablePackages<'cache_root, 'lock_file> {
    pub(crate) fn from(
        cache_root: &'cache_root Path,
        phantom: PhantomData<&'lock_file LockFileOwnership>,
    ) -> Self {
        AvailablePackages {
            cache_root,
            phantom,
        }
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
    pub fn update(&self, package: &PackageManifest) -> Result<(), Error> {
        let cache_path = self
            .cache_root
            .join(package.repository().as_str())
            .join(package.category().as_str())
            .join(package.name().as_str());

        let res: Result<_, Error> = try {
            if let Some(parent) = cache_path.parent() {
                fs::create_dir_all(parent)?;
            }

            let mut file = File::create(&cache_path)?;
            file.write_all(serde_json::to_string_pretty(package)?.as_bytes())?;
            file.write_all(&[b'\n'])?;
        };
        res.context(cache_path.display().to_string())
            .context(CacheErrorKind::CacheWriteError)?;
        Ok(())
    }

    /// Returns an [`AvailablePackagesCacheQuery`] allowing to browse the cache according to the given [`PackageRequirement`]
    #[inline]
    pub fn query<'pkg_req>(
        &self,
        requirement: &'pkg_req SoftPackageRequirement,
    ) -> AvailablePackagesCacheQuery<'cache_root, 'pkg_req> {
        AvailablePackagesCacheQuery::from(&self.cache_root, requirement)
    }
}

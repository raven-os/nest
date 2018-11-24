//! Handles and methods to process the cache of available packages.

mod query;
pub use self::query::{AvailablePackagesCacheQuery, AvailablePackagesCacheQueryStrategy};

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use failure::{Error, ResultExt};
use serde_json;

use crate::package::{Package, PackageRequirement};
use crate::repository::Repository;

/// A handle on the cache of available packages, to perform searches or to erase it.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct AvailablePackages<'a> {
    base: &'a Path,
}

impl<'a> AvailablePackages<'a> {
    #[inline]
    pub(crate) fn from(base: &Path) -> AvailablePackages {
        AvailablePackages { base }
    }

    /// Erases the whole cache.
    #[inline]
    pub fn erase(&self) -> Result<(), Error> {
        if self.base.exists() {
            fs::remove_dir_all(self.base).with_context(|_| self.base.display().to_string())?;
        }
        Ok(())
    }

    /// Erases the cache of a given repository
    #[inline]
    pub fn erase_repository(&self, repository: &Repository) -> Result<(), Error> {
        let path = self.base.join(repository.name());
        if path.exists() {
            fs::remove_dir_all(&path).with_context(|_| path.display().to_string())?;
        }
        Ok(())
    }

    /// Returns an [`AvailablePackagesCacheQuery`][1] on a search through this cache following the given requirement.
    ///
    /// [1]: struct.AvailablePackagesCacheQuery.html
    #[inline]
    pub fn search<'b>(
        &self,
        requirement: &'b PackageRequirement,
    ) -> AvailablePackagesCacheQuery<'a, 'b> {
        AvailablePackagesCacheQuery::from(self.base, requirement)
    }

    /// Updates the cache entry of the given [`Package`][1] with the given metadatas, creating it if it didn't exist yet.
    ///
    /// [1]: ../../package/struct.Package.html
    pub fn update(&self, package: &Package) -> Result<(), Error> {
        let metadata = package.manifest().metadata();

        let cache_path = self
            .base
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
        res.with_context(|_| cache_path.display().to_string())?;
        Ok(())
    }
}

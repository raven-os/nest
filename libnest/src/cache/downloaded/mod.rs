//! Module to query and manipulate the cache of downloaded packages

use std::fs;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

use data_encoding::HEXUPPER;
use sha2::{Digest, Sha256};

use crate::cache::{CacheError, CacheErrorKind};
use crate::lock_file::LockFileOwnership;
use crate::package::{NPFExplorationError, NPFExplorer, PackageID};

/// Structure representing the cache of downloaded packages
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct DownloadedPackages<'cache_root, 'lock_file> {
    cache_root: &'cache_root Path,
    phantom: PhantomData<&'lock_file LockFileOwnership>,
}

impl<'cache_root, 'lock_file> DownloadedPackages<'cache_root, 'lock_file> {
    pub(crate) fn from(
        cache_root: &'cache_root Path,
        phantom: PhantomData<&'lock_file LockFileOwnership>,
    ) -> Self {
        Self {
            cache_root,
            phantom,
        }
    }

    fn package_path(&self, package: &PackageID) -> PathBuf {
        self.cache_root
            .join(package.repository().as_str())
            .join(package.category().as_str())
            .join(package.name().as_str())
            .join(format!("{}-{}.nest", package.name(), package.version()))
    }

    /// Checks whether a given package has already been downloaded
    pub fn has_package(&self, package: &PackageID) -> bool {
        self.package_path(package).exists()
    }

    /// Checks whether a given package has already been downloaded and matches a given hash
    pub fn has_package_matching_hash(
        &self,
        package: &PackageID,
        hash: &str,
    ) -> Result<bool, CacheError> {
        let package_path = self.package_path(package);

        if !package_path.exists() {
            return Ok(false);
        }

        fs::File::open(package_path)
            .and_then(|mut file| {
                let mut sha256 = Sha256::default();
                std::io::copy(&mut file, &mut sha256)
                    .map(|_| HEXUPPER.encode(sha256.result().as_ref()) == hash)
            })
            .map_err(|_| CacheError::from(CacheErrorKind::CacheLoadError))
    }

    /// Opens a downloaded package for exploration
    pub fn explore_package(&self, package: &PackageID) -> Result<NPFExplorer, NPFExplorationError> {
        NPFExplorer::from(self.package_path(package))
    }

    /// Removes the NPF for a given package
    pub fn remove_package(&self, package: &PackageID) -> Result<(), std::io::Error> {
        let path = self.package_path(package);

        fs::remove_file(&path)
    }
}

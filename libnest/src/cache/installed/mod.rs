//! Module to query and manipulate the cache of installed packages

pub mod log;

use std::fs;
use std::marker::PhantomData;
use std::path::Path;

use crate::lock_file::LockFileOwnership;
use crate::package::PackageID;

use self::log::Log;

/// Structure representing the cache of installed packages
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct InstalledPackages<'cache_root, 'lock_file> {
    cache_root: &'cache_root Path,
    phantom: PhantomData<&'lock_file LockFileOwnership>,
}

impl<'cache_root, 'lock_file> InstalledPackages<'cache_root, 'lock_file> {
    pub(crate) fn from(
        cache_root: &'cache_root Path,
        phantom: PhantomData<&'lock_file LockFileOwnership>,
    ) -> Self {
        Self {
            cache_root,
            phantom,
        }
    }

    /// Loads the log of installed files for a given package
    pub fn package_log(&self, package: &PackageID) -> Result<Log, std::io::Error> {
        let path = self
            .cache_root
            .join(package.repository().as_str())
            .join(package.category().as_str())
            .join(package.name().as_str())
            .join(package.version().to_string());

        Log::load_from_file(path)
    }

    /// Saves the log of installed files for a given package
    pub fn save_package_log(&self, package: &PackageID, log: &Log) -> Result<(), std::io::Error> {
        let log_dir = self
            .cache_root
            .join(package.repository().as_str())
            .join(package.category().as_str())
            .join(package.name().as_str());
        fs::create_dir_all(&log_dir)?;

        let path = log_dir.join(package.version().to_string());

        log.save_to_file(path)
    }

    /// Removes the log of installed files for a given package
    pub fn remove_package_log(&self, package: &PackageID) -> Result<(), std::io::Error> {
        let path = self
            .cache_root
            .join(package.repository().as_str())
            .join(package.category().as_str())
            .join(package.name().as_str())
            .join(package.version().to_string());

        fs::remove_file(&path)
    }
}

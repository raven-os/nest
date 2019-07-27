use std::fs;
use std::path::Path;

use failure::ResultExt;

use crate::chroot::Chroot;
use crate::config::Config;
use crate::lock_file::LockFileOwnership;
use crate::package::{Kind, NPFExplorer, PackageID};

use super::{RemoveError, RemoveErrorKind::*};

/// Structure representing a "remove" transaction
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct RemoveTransaction {
    target: PackageID,
}

impl RemoveTransaction {
    /// Creates a [`RemoveTransaction`] from a given [`PackageID`]
    pub fn from(target: PackageID) -> Self {
        RemoveTransaction { target }
    }

    /// Returns the target [`PackageID`] for this transaction
    pub fn target(&self) -> &PackageID {
        &self.target
    }

    /// Performs the removal of the package
    pub fn perform(
        &self,
        config: &Config,
        lock_ownership: &LockFileOwnership,
    ) -> Result<(), RemoveError> {
        let downloaded_packages = config.downloaded_packages_cache(lock_ownership);
        let npf_explorer = downloaded_packages
            .explore_package(self.target())
            .map_err(|_| InvalidCachedPackageFile)?;

        remove_package(config, lock_ownership, npf_explorer, self.target())
    }
}

fn is_empty_directory(dir_path: &Path) -> std::io::Result<bool> {
    let mut it = fs::read_dir(dir_path)?;

    Ok(it.next().is_none())
}

/// Remove the package from a given [`NPFExplorer`], using a given [`PackageID`]'s log
pub(crate) fn remove_package(
    config: &Config,
    lock_ownership: &LockFileOwnership,
    npf_explorer: NPFExplorer,
    target_id: &PackageID,
) -> Result<(), RemoveError> {
    let instructions_handle = npf_explorer
        .load_instructions()
        .map_err(|_| InvalidCachedPackageFile)?;

    if let Some(executor) = &instructions_handle {
        executor
            .execute_before_remove(config.paths().root())
            .map_err(PreRemoveInstructionsFailure)?;
    }

    // If the package is effective, installed files must be removed
    if npf_explorer.manifest().kind() == Kind::Effective {
        // Open the log file, and remove all the files listed in it
        let log = config
            .installed_packages_cache(lock_ownership)
            .package_log(target_id)
            .map_err(LogFileLoadError)?;

        // Iterate backwards to ensure removal of nested files before that of top-level directories
        for entry in log.files().into_iter().rev() {
            let abs_path = Path::new("/").with_content(entry.path());
            let rel_path = config.paths().root().with_content(entry.path());

            if let Ok(metadata) = fs::symlink_metadata(&rel_path) {
                match (entry.file_type().is_dir(), metadata.file_type().is_dir()) {
                    // The file to remove is a directory, remove it if it is empty
                    (true, true) => {
                        if let Ok(true) = is_empty_directory(&rel_path) {
                            fs::remove_dir(&rel_path)
                        } else {
                            Ok(())
                        }
                    }

                    // The file was expected to be a directory, but is a symlink, leave it
                    (true, false) if metadata.file_type().is_symlink() => Ok(()),

                    // The file to remove is a regular file, remove it
                    _ => fs::remove_file(&rel_path),
                }
                .with_context(|_| FileRemoveError(abs_path))?;
            }
        }

        config
            .installed_packages_cache(lock_ownership)
            .remove_package_log(target_id)
            .with_context(|_| target_id.to_string())
            .with_context(|_| LogFileRemoveError)?;
    }

    if let Some(executor) = &instructions_handle {
        executor
            .execute_after_remove(config.paths().root())
            .map_err(PostRemoveInstructionsFailure)?;
    }

    Ok(())
}

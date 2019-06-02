use std::fs::{self, File};
use std::io::{BufRead, BufReader, Seek, SeekFrom};
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
    pub fn perform(&self, config: &Config, _: &LockFileOwnership) -> Result<(), RemoveError> {
        let npf_path = config
            .paths()
            .downloaded()
            .join(self.target().repository().as_str())
            .join(self.target().category().as_str())
            .join(self.target().name().as_str())
            .join(format!(
                "{}-{}.nest",
                self.target().name(),
                self.target().version()
            ));

        let npf_explorer = NPFExplorer::from(&npf_path).map_err(|_| InvalidCachedPackageFile)?;

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
            // Get the log file of the target package
            let log_path = config
                .paths()
                .installed()
                .join(self.target().repository().as_str())
                .join(self.target().category().as_str())
                .join(self.target().name().as_str())
                .join(self.target.version().to_string());

            let mut log_file = File::open(&log_path)
                .with_context(|_| log_path.display().to_string())
                .with_context(|_| LogFileLoadError)?;

            // Remove all the files listed in the log file
            log_file
                .seek(SeekFrom::Start(0))
                .with_context(|_| log_path.display().to_string())
                .with_context(|_| LogFileLoadError)?;

            for entry_path in BufReader::new(&log_file).lines() {
                let entry_path = entry_path.map_err(|_| LogFileLoadError)?;
                let abs_path = Path::new("/").with_content(&entry_path);
                let rel_path = config.paths().root().with_content(&entry_path);

                if let Ok(metadata) = fs::symlink_metadata(&rel_path) {
                    if !metadata.is_dir() {
                        fs::remove_file(&rel_path).with_context(|_| FileRemoveError(abs_path))?;
                    }
                }
            }

            fs::remove_file(&log_path)
                .with_context(|_| log_path.display().to_string())
                .with_context(|_| LogFileRemoveError)?;
        }

        if let Some(executor) = &instructions_handle {
            executor
                .execute_after_remove(config.paths().root())
                .map_err(PostRemoveInstructionsFailure)?;
        }

        Ok(())
    }
}

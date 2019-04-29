use std::fs::{self, File};
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;

use failure::{Error, ResultExt};

use crate::chroot::Chroot;
use crate::config::Config;
use crate::lock_file::LockFileOwnership;
use crate::package::PackageID;

use super::RemoveErrorKind;

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
    pub fn perform(&self, config: &Config, _: &LockFileOwnership) -> Result<(), Error> {
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
            .with_context(|_| RemoveErrorKind::LogFileLoadError)?;

        // Remove all the files listed in the log file, and directories if they are empty
        log_file
            .seek(SeekFrom::Start(0))
            .with_context(|_| log_path.display().to_string())
            .with_context(|_| RemoveErrorKind::LogFileLoadError)?;

        for entry_path in BufReader::new(&log_file).lines() {
            let entry_path = entry_path?;
            let abs_path = Path::new("/").with_content(&entry_path);
            let rel_path = config.paths().root().with_content(&entry_path);

            if let Ok(metadata) = fs::symlink_metadata(&rel_path) {
                if !metadata.is_dir() {
                    fs::remove_file(&rel_path)
                        .with_context(|_| abs_path.display().to_string())
                        .with_context(|_| RemoveErrorKind::FileRemoveError)?;
                }
            }
        }

        fs::remove_file(&log_path)
            .with_context(|_| log_path.display().to_string())
            .with_context(|_| RemoveErrorKind::LogFileRemoveError)?;

        Ok(())
    }
}

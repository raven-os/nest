use std::fs::{self, File};
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;

use failure::{Error, ResultExt};

use config::Config;
use package::PackageId;
use transaction::{Notifier, Notification, Transaction, TransactionKind, TransactionStep};
use chroot::Chroot;

/// A `remove` transaction: it performs the removal of the target on the system.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Remove {
    target: PackageId,
    target_name: String,
    idx: usize,
}

impl Remove {
    /// Creates a [`Remove`] from a target [`PackageId`].
    #[inline]
    pub fn from(target: PackageId) -> Remove {
        let target_name = target.to_string();
        Remove {
            target,
            target_name,
            idx: 0,
        }
    }
}

impl Transaction for Remove {
    #[inline]
    fn idx(&self) -> usize {
        self.idx
    }

    #[inline]
    fn assign_idx(&mut self, idx: usize) {
        self.idx = idx;
    }

    #[inline]
    fn kind(&self) -> TransactionKind {
        TransactionKind::Remove
    }

    #[inline]
    fn target(&self) -> &str {
        &self.target_name
    }

    fn perform(&mut self, config: &Config, notifier: &mut Notifier) -> Result<(), Error> {
        // Step 1: prepare the removal
        notifier.notify(self, Notification::NewStep(TransactionStep::Prepare, false));

        // Get the log file of the target package
        let log_path = config
            .paths()
            .installed()
            .join(self.target.full_name().repository())
            .join(self.target.full_name().category())
            .join(self.target.full_name().name())
            .join(self.target.version().to_string())
        ;

        let mut log_file = File::open(&log_path).with_context(|_| log_path.display().to_string())?;

        // Count the number of files to remove (for progress)
        let mut nb_files = BufReader::new(&log_file).lines().count();
        nb_files += 1; // The log file must be removed to.

        // Remove all the files in the log file. Remove directory if they are empty
        log_file.seek(SeekFrom::Start(0))
                .with_context(|_| log_path.display().to_string())?;
        for (i, entry_path) in BufReader::new(&log_file).lines().enumerate() {
            let entry_path = entry_path?;
            let abs_path = Path::new("/").with_content(&entry_path);
            let rel_path = config.paths().root().with_content(&entry_path);

            if let Ok(metadatas) = fs::symlink_metadata(&rel_path) {
                if metadatas.is_dir() {
                    println!("Trying to remove {}", rel_path.display());
                    let res = fs::remove_dir(&rel_path); // Ignore errors so it doesn't stop on non-empty directory
                    if res.is_ok() {
                        println!("Removed {}.", rel_path.display());
                    }
                } else {
                    fs::remove_file(rel_path)
                        .with_context(|_| abs_path.display().to_string())
                    ?;
                }
            }
            notifier.notify(self, Notification::Progress(i, nb_files));
        }
        fs::remove_file(&log_path)
            .with_context(|_| log_path.display().to_string())
        ?;
        Ok(())
    }
}

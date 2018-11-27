use std::fs::{self, File};
use std::io::{Seek, SeekFrom, Write};
use std::path::Path;

use failure::{Error, ResultExt};
use flate2::read::GzDecoder;
use tar::Archive;

use crate::chroot::Chroot;
use crate::config::Config;
use crate::error::InstallError;
use crate::package::PackageId;
use crate::transaction::{Notification, Notifier, Transaction, TransactionKind, TransactionStep};

/// An `install` transaction: it performs the installation of the target on the system.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Install {
    target: PackageId,
    target_name: String,
    idx: usize,
}

impl Install {
    /// Creates an [`Install`] from a target [`PackageId`].
    #[inline]
    pub fn from(target: PackageId) -> Install {
        let target_name = target.to_string();
        Install {
            target,
            target_name,
            idx: 0,
        }
    }
}

impl Transaction for Install {
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
        TransactionKind::Install
    }

    #[inline]
    fn target(&self) -> &str {
        &self.target_name
    }

    fn perform(&mut self, config: &Config, notifier: &mut Notifier) -> Result<(), Error> {
        let mut files = Vec::new();

        // Step 1: prepare the installation
        notifier.notify(self, Notification::NewStep(TransactionStep::Prepare, false));

        // Find the repository of the package we want to install
        let repository = config
            .repositories()
            .into_iter()
            .find(|repository| repository.name() == self.target.full_name().repository())
            .ok_or_else(|| {
                InstallError::CantFindRepository(
                    self.target.full_name().repository().to_string(),
                    self.target.full_name().repository().to_string(),
                )
            })?;

        // Build target URL
        let target_url = format!(
            "/p/{}/{}/{}/download",
            self.target.full_name().category(),
            self.target.full_name().name(),
            self.target.version(),
        );

        // Build target folder and destination path
        let tarball_path = config
            .paths()
            .downloaded()
            .join(self.target.full_name().repository())
            .join(self.target.full_name().category())
            .join(self.target.full_name().name())
            .join(self.target.version().to_string());
        fs::create_dir_all(&tarball_path).with_context(|_| tarball_path.display().to_string())?;
        let tarball_path = tarball_path.join("data").with_extension("tar.gz");

        // Step 2: Download the package
        let mut tarball =
            File::create(&tarball_path).with_context(|_| tarball_path.display().to_string())?;
        let mut transfer = repository.transfer(config).target(target_url);
        transfer.perform(&mut tarball, self, notifier)?;
        tarball
            .flush()
            .with_context(|_| tarball_path.display().to_string())?;

        // Calculate number of files to install (for progress)
        let mut tarball =
            File::open(&tarball_path).with_context(|_| tarball_path.display().to_string())?;
        let res: Result<_, Error> = try {
            tarball.seek(SeekFrom::Start(0))?;

            let mut archive = Archive::new(GzDecoder::new(&tarball));
            archive.entries()?.count()
        };
        let nb_files = res.with_context(|_| tarball_path.display().to_string())?;

        // Step 3: Check that the package isn't already installed. This shouldn't be needed, as it should have been checked before.
        notifier.notify(self, Notification::NewStep(TransactionStep::Check, false));

        // Test if the log file of the package exists. If it's the case, the package is already installed
        let log_dir = config
            .paths()
            .installed()
            .join(self.target.full_name().repository())
            .join(self.target.full_name().category())
            .join(self.target.full_name().name());
        fs::create_dir_all(&log_dir).with_context(|_| log_dir.display().to_string())?;

        let log_path = log_dir.join(self.target.version().to_string());
        if log_path.exists() {
            Err(InstallError::PackageAlreadyInstalled(
                self.target.to_string(),
            ))?
        }

        // Step 4: Check that the installation will not overwrite any existing file
        {
            tarball
                .seek(SeekFrom::Start(0))
                .with_context(|_| tarball_path.display().to_string())?;
            let mut archive = Archive::new(GzDecoder::new(&tarball));

            for (i, entry) in archive
                .entries()
                .with_context(|_| tarball_path.display().to_string())?
                .enumerate()
            {
                let entry = entry.with_context(|_| tarball_path.display().to_string())?;
                let entry_path = entry
                    .path()
                    .with_context(|_| tarball_path.display().to_string())?;

                let abs_path = Path::new("/").with_content(&entry_path);
                let rel_path = config.paths().root().with_content(&entry_path);

                if let Ok(metadatas) = fs::symlink_metadata(&rel_path) {
                    if !metadatas.is_dir() {
                        Err(InstallError::FileAlreadyExists(
                            abs_path.display().to_string(),
                        ))?;
                    }
                }
                files.push(abs_path.to_path_buf());
                notifier.notify(self, Notification::Progress(i, nb_files));
            }
        }

        let res: Result<_, Error> = try {
            // Step 5: Fill the log file with the files
            let mut log = File::create(&log_path)?;
            for file in &files {
                writeln!(log, "{}", file.display())?;
            }

            // Step 6: Extract the tarball in the root folder
            notifier.notify(self, Notification::NewStep(TransactionStep::Extract, false));
            tarball.seek(SeekFrom::Start(0))?;
            let mut archive = Archive::new(GzDecoder::new(&tarball));
            for (i, entry) in archive.entries()?.enumerate() {
                entry?.unpack_in(config.paths().root())?;
                notifier.notify(self, Notification::Progress(i, nb_files));
            }
            ()
        };
        res.with_context(|_| tarball_path.display().to_string())?;
        Ok(())
    }
}

use std::fs::{self, File};
use std::io::{Seek, SeekFrom, Write};
use std::path::Path;

use failure::{format_err, Error, ResultExt};
use flate2::read::GzDecoder;
use tar::Archive;

use crate::chroot::Chroot;
use crate::config::Config;
use crate::lock_file::LockFileOwnership;
use crate::package::{NPFExplorer, PackageID};

use super::InstallErrorKind;

/// Structure representing an "install" transaction
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct InstallTransaction {
    target: PackageID,
}

impl InstallTransaction {
    /// Creates an [`InstallTransaction`] from a given [`PackageID`]
    #[inline]
    pub fn from(target: PackageID) -> Self {
        InstallTransaction { target }
    }

    /// Returns the target [`PackageID`] for this transaction
    pub fn target(&self) -> &PackageID {
        &self.target
    }

    /// Creates the download file and returns a handle to it
    pub fn create_download_file(&self, config: &Config) -> Result<(impl Write + Seek), Error> {
        // Create target folder and destination file
        let npf_path = config
            .paths()
            .downloaded()
            .join(self.target().repository().as_str())
            .join(self.target().category().as_str())
            .join(self.target().name().as_str());
        fs::create_dir_all(&npf_path).with_context(|_| npf_path.display().to_string())?;
        let tarball_path = npf_path.join(format!(
            "{}-{}.nest",
            self.target().name(),
            self.target().version()
        ));

        // Open the destination file and return it as the writer handle
        let tarball_file =
            File::create(&tarball_path).with_context(|_| tarball_path.display().to_string())?;
        Ok(tarball_file)
    }

    /// Extracts the downloaded file and performs the installation
    pub fn extract(&self, config: &Config, _: &LockFileOwnership) -> Result<(), Error> {
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

        let npf_explorer = NPFExplorer::from(self.target().name(), &npf_path)?;

        // TODO: avoid failing if no tarball is found and the package is virtual
        let tarball_handle = npf_explorer
            .get_data()
            .with_context(|_| npf_path.display().to_string())?;

        if let Some(tarball_handle) = tarball_handle {
            let mut tarball = tarball_handle.file();
            let mut archive = Archive::new(GzDecoder::new(tarball));
            let mut files = Vec::new();

            // List all the files in the archive and check whether they already exist
            for entry in archive
                .entries()
                .with_context(|_| npf_path.display().to_string())?
            {
                let entry = entry.with_context(|_| npf_path.display().to_string())?;
                let entry_path = entry
                    .path()
                    .with_context(|_| npf_path.display().to_string())?;

                let abs_path = Path::new("/").with_content(&entry_path);
                let rel_path = config.paths().root().with_content(&entry_path);

                // If the file exists and is not a directory, the installation would overwrite an
                // existing file, return an error.
                if let Ok(metadata) = fs::symlink_metadata(&rel_path) {
                    if !metadata.is_dir() {
                        Err(format_err!("{}", abs_path.display())
                            .context(InstallErrorKind::FileAlreadyExists))?;
                    }
                }
                files.push(abs_path.to_path_buf());
            }

            let log_dir = config
                .paths()
                .installed()
                .join(self.target().repository().as_str())
                .join(self.target().category().as_str())
                .join(self.target().name().as_str());
            fs::create_dir_all(&log_dir).with_context(|_| log_dir.display().to_string())?;

            let log_path = log_dir.join(self.target.version().to_string());

            // If the log file exists, the package is already installed
            if log_path.exists() {
                Err(format_err!("{}", &self.target)
                    .context(InstallErrorKind::PackageAlreadyInstalled))?;
            }

            let res: Result<_, Error> = try {
                // Log each file to install to the log file
                let mut log = File::create(&log_path)?;
                for file in &files {
                    writeln!(log, "{}", file.display())?;
                }

                // Extract the tarball in the root folder
                tarball.seek(SeekFrom::Start(0))?;
                let mut archive = Archive::new(GzDecoder::new(tarball));
                for entry in archive.entries()? {
                    entry?.unpack_in(config.paths().root())?;
                }
            };
            res.with_context(|_| npf_path.display().to_string())?;
        }

        Ok(())
    }
}

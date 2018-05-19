//! Types to install a package on the targeted system.
//!
//! This module provides the struct [`Installer`]  to perform an installation of a package on the targeted system.
//! The installation is divided into steps (See [`InstallState`]):
//!     * Waiting
//!     * Check
//!     * Install

use std::fmt::{self, Display, Formatter};
use std::fs::{self, File};
use std::io::{Seek, SeekFrom, Write};
use std::path::Path;

use failure::{Error, ResultExt};
use flate2::read::GzDecoder;
use tar::Archive;

use chroot::Chroot;
use config::Config;
use error::InstallErrorKind;
use package::Package;
use system::System;

#[allow(missing_docs)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
/// All the states the installer can be at.
pub enum InstallState {
    Waiting,
    Check,
    Prepare,
    Install,
}

impl Display for InstallState {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match *self {
            InstallState::Waiting => write!(f, "waiting"),
            InstallState::Check => write!(f, "check"),
            InstallState::Prepare => write!(f, "prepare"),
            InstallState::Install => write!(f, "install"),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
/// An installer to install packages on a targeted system.
pub struct Installer<'a, 'b, 'c, 'd> {
    state: InstallState,
    system: &'a System,
    config: &'b Config,
    data: &'c Path,
    package: &'d Package<'d>,
}

impl<'a, 'b, 'c, 'd> Installer<'a, 'b, 'c, 'd> {
    pub(crate) fn from(
        system: &'a System,
        config: &'b Config,
        data: &'c Path,
        package: &'d Package<'d>,
    ) -> Installer<'a, 'b, 'c, 'd> {
        Installer {
            state: InstallState::Waiting,
            system,
            config,
            data,
            package,
        }
    }

    /// Performs the installation.
    pub fn perform<F>(&mut self, mut cb: F) -> Result<(), Error>
    where
        F: FnMut(InstallState, Option<(usize, usize)>),
    {
        // We'll use ChrootPath instead of PathBuf in this function
        let dest_path = self.system.install_path();

        // Check the existence and validity of destination directory
        if !dest_path.exists() || !dest_path.is_dir() {
            Err(InstallErrorKind::DestFolderError(
                dest_path.display().to_string(),
            ))?;
        }

        let mut tarball = File::open(self.data)?;
        let mut files = Vec::new();

        // Step 1: Check that the package isn't already installed
        cb(InstallState::Check, None);
        let log_path = {
            let mut content_path = self.config
                .installed()
                .join(self.package.repository().name())
                .join(self.package.manifest().metadata().category());

            let log_dir = dest_path.with_content(&content_path);
            fs::create_dir_all(&log_dir).context(log_dir.display().to_string())?;

            content_path.push(self.package.manifest().metadata().name());
            let log_path = dest_path.with_content(content_path);

            // If the log file exists, then the package is already installed
            if log_path.exists() {
                Err(InstallErrorKind::PackageAlreadyInstalled)?;
            }
            log_path
        };

        // Step 2: check that the files we want to install will not overwrite any existing ones
        {
            let mut archive = Archive::new(GzDecoder::new(&tarball));
            for entry in archive.entries()? {
                let entry = entry?;
                let relative_path = entry.path()?;
                let path = dest_path.with_content(&relative_path);
                if !path.is_dir() && path.exists() {
                    Err(InstallErrorKind::FileAlreadyExists(
                        path.display().to_string(),
                    ))?;
                }
                files.push(Path::new("/").with_content(relative_path.to_path_buf())); // Absolute path here, no relative ones
            }
        }

        // Step 3: Fill the log file with the installed files BEFORE INSTALLATION so we can remove
        // the package if the installation is cancelled or if it failed
        //
        // The log file is used to know which files should be remove when uninstalling the package
        {
            let mut log = File::create(&log_path).context(log_path.display().to_string())?;
            cb(InstallState::Prepare, None);
            for file in files.iter() {
                writeln!(log, "{}", file.display()).context(log_path.display().to_string())?;
            }
        }

        // Step 4: extract the data to the destination folder (usually '/'), and fill the log file
        // We're not using `archive.unpack_in()` because we want to be sure that extraction will behave the same way
        // than when we filled the log file
        {
            tarball.seek(SeekFrom::Start(0))?;
            let mut archive = Archive::new(GzDecoder::new(&tarball));
            for (i, entry) in archive.entries()?.enumerate() {
                cb(InstallState::Install, Some((i, files.len())));
                entry?.unpack_in(&dest_path)?;
            }
        }
        Ok(())
    }
}

//! Types to install a package on the targeted system.
//!
//! This module provides the struct [`Installer`]  to perform an installation of a package on the targetted system.
//! The installation is divided into steps (See [`InstallState`]):
//!     * Waiting
//!     * Extract
//!     * Check
//!     * Install

use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::{Seek, SeekFrom};
use std::path::Path;

use failure::Error;
use flate2::read::GzDecoder;
use tar::Archive;

use error::InstallErrorKind;
use package::Manifest;
use system::System;

#[allow(missing_docs)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
/// All the states the installer can be at.
pub enum InstallState {
    Waiting,
    Extract,
    Check,
    Install,
}

impl Display for InstallState {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match *self {
            InstallState::Waiting => write!(f, "waiting"),
            InstallState::Extract => write!(f, "extract"),
            InstallState::Check => write!(f, "check"),
            InstallState::Install => write!(f, "install"),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
/// An installer to install packages on a targeted system.
pub struct Installer<'a, 'b, 'c> {
    state: InstallState,
    system: &'a System,
    data: &'b Path,
    manifest: &'c Manifest,
}

impl<'a, 'b, 'c> Installer<'a, 'b, 'c> {
    pub(crate) fn from(
        system: &'a System,
        data: &'b Path,
        manifest: &'c Manifest,
    ) -> Installer<'a, 'b, 'c> {
        Installer {
            state: InstallState::Waiting,
            system,
            data,
            manifest,
        }
    }

    /// Performs the installation
    pub fn perform<F>(&mut self, mut cb: F) -> Result<(), Error>
    where
        F: FnMut(InstallState, Option<(usize, usize)>),
    {
        let dest_path = self.system.install_path();

        // Open the file one time at the beginning
        let mut file = File::open(self.data)?;
        let mut entries = 0;

        // First step: check that the files we want to install will not overwrite any existing ones
        {
            cb(InstallState::Check, None);
            let mut data = Archive::new(GzDecoder::new(&file));
            for entry in data.entries()? {
                let entry = entry?;
                let path = dest_path.join(&entry.path()?);
                if !path.is_dir() && path.exists() {
                    Err(InstallErrorKind::FileAlreadyExists(
                        path.display().to_string(),
                    ))?;
                }
                entries += 1;
            }
        }

        // Second step: extract the data to the destination folder (usually '/').
        {
            file.seek(SeekFrom::Start(0))?;
            let mut data = Archive::new(GzDecoder::new(&file));
            for (i, entry) in data.entries()?.enumerate() {
                cb(InstallState::Install, Some((i, entries)));
                let mut entry = entry?;
                entry.unpack_in(&dest_path)?;
            }
        }
        Ok(())
    }
}

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
pub struct Installer<'a, 'b> {
    state: InstallState,
    data: &'a Path,
    manifest: &'b Manifest,
}

impl<'a, 'b> Installer<'a, 'b> {
    pub(crate) fn from(data: &'a Path, manifest: &'b Manifest) -> Installer<'a, 'b> {
        Installer {
            state: InstallState::Waiting,
            data,
            manifest,
        }
    }

    /// Performs the installation
    pub fn perform<F>(&mut self, mut cb: F) -> Result<(), Error>
    where
        F: FnMut(InstallState, Option<(usize, usize)>),
    {
        // DEBUG DATA
        use std::path::PathBuf;
        let dest_path = PathBuf::from("./dest");

        // Open the file one time at the beginning
        let mut file = File::open(self.data)?;
        let mut entries = 0;

        // First step: check that the files we want to install will not overwrite any existing one
        {
            cb(InstallState::Check, None);
            let mut datas = Archive::new(GzDecoder::new(&file));
            for entry in datas.entries()? {
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
            let mut datas = Archive::new(GzDecoder::new(&file));
            for (i, entry) in datas.entries()?.enumerate() {
                cb(InstallState::Install, Some((i, entries)));
                let mut entry = entry?;
                entry.unpack_in(&dest_path)?;
            }
        }
        Ok(())
    }
}

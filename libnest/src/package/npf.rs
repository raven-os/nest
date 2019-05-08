use std::fs::{self, File};
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

use failure::Error;
use tar::Archive;

use super::error::{NPFExplorationError, NPFExplorationErrorKind};

/// Structure representing a handle over a file contained in an NPF
#[derive(Debug)]
pub struct NPFFile<'explorer> {
    file: File,
    phantom: PhantomData<&'explorer NPFExplorer>,
}

impl<'explorer> NPFFile<'explorer> {
    pub(crate) fn from(file: File, phantom: PhantomData<&'explorer NPFExplorer>) -> Self {
        Self { file, phantom }
    }

    /// Retrieves the file associated with this handle
    pub fn file(&self) -> &File {
        &self.file
    }
}

/// Structure representing an NPF to allow interacting with it
#[derive(Debug)]
pub struct NPFExplorer {
    path: PathBuf,
}

impl NPFExplorer {
    /// Create a NPFExplorer from a name and a path to an NPF archive
    pub fn from(name: &str, npf_path: &Path) -> Result<Self, Error> {
        let path = std::env::temp_dir().join(name);

        // Create a directory to extract the NPF
        fs::create_dir(&path)?;

        // Unpack the NPF
        let file = File::open(npf_path)?;
        let mut archive = Archive::new(&file);
        archive.unpack(&path)?;

        Ok(Self { path })
    }

    /// Retrieves a handle over a file in the NPF
    fn get_file<P: AsRef<Path> + ToString + Copy>(
        &self,
        path: P,
    ) -> Result<NPFFile, NPFExplorationError> {
        let file = File::open(self.path.join(path)).map_err(|err| match err.kind() {
            std::io::ErrorKind::NotFound => NPFExplorationErrorKind::FileNotFound(path.to_string()),
            _ => NPFExplorationErrorKind::FileOpenError(path.to_string()),
        })?;

        Ok(NPFFile::from(file, PhantomData))
    }

    /// Retrieves a handle over the NPF's manifest.toml
    pub fn get_manifest(&self) -> Result<NPFFile, NPFExplorationError> {
        self.get_file("manifest.toml")
    }

    /// Retrieves a handle over the NPF's data.tar.gz
    pub fn get_data(&self) -> Result<NPFFile, NPFExplorationError> {
        self.get_file("data.tar.gz")
    }

    /// Retrieves a handle over the NPF's instructions.sh, if one exists
    pub fn get_instructions(&self) -> Result<Option<NPFFile>, NPFExplorationError> {
        self.get_file("instructions.sh").map_or_else(
            |e| match e.kind() {
                NPFExplorationErrorKind::FileNotFound(_) => Ok(None),
                _ => Err(e.into()),
            },
            |o| Ok(Some(o)),
        )
    }
}

impl Drop for NPFExplorer {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.path).expect("unable to cleanup an extracted NPF");
    }
}

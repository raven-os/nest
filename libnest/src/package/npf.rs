use std::fs::{self, File};
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

use failure::Error;
use tar::Archive;

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
    fn get_file<P: AsRef<Path>>(&self, path: P) -> Result<NPFFile, Error> {
        let file = File::open(self.path.join(path))?;

        Ok(NPFFile::from(file, PhantomData))
    }

    /// Retrieves a handle over the NPF's manifest.toml
    pub fn get_manifest(&self) -> Result<NPFFile, Error> {
        self.get_file("manifest.toml")
    }

    /// Retrieves a handle over the NPF's data.tar.gz
    pub fn get_data(&self) -> Result<NPFFile, Error> {
        self.get_file("data.tar.gz")
    }
}

impl Drop for NPFExplorer {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.path).expect("unable to cleanup an extracted NPF");
    }
}

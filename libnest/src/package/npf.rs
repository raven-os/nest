use std::fs::{self, File};
use std::io::Read;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

use tar::Archive;
use toml;

use super::error::{NPFExplorationError, NPFExplorationErrorKind};
use super::manifest::{Kind::Effective, Manifest};

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
    manifest: Manifest,
    path: PathBuf,
}

impl NPFExplorer {
    fn load_manifest(path: &Path) -> Result<Manifest, NPFExplorationError> {
        let mut file = File::open(path.join("manifest.toml")).map_err(|err| match err.kind() {
            std::io::ErrorKind::NotFound => NPFExplorationErrorKind::MissingManifest,
            _ => NPFExplorationErrorKind::FileIOError(path.to_path_buf()),
        })?;

        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|_| NPFExplorationErrorKind::FileIOError(path.to_path_buf()))?;

        Ok(toml::from_str(&content).map_err(|_| NPFExplorationErrorKind::InvalidManifest)?)
    }

    /// Create a NPFExplorer from a name and a path to an NPF archive
    pub fn from(name: &str, npf_path: &Path) -> Result<Self, NPFExplorationError> {
        let path = std::env::temp_dir().join(name);

        // Create a directory to extract the NPF
        fs::create_dir(&path).map_err(|_| NPFExplorationErrorKind::UnpackError)?;

        // Unpack the NPF
        File::open(npf_path)
            .and_then(|file| {
                let mut archive = Archive::new(&file);
                archive.unpack(&path)
            })
            .map_err(|_| NPFExplorationErrorKind::UnpackError)?;

        let manifest = Self::load_manifest(&path)?;

        Ok(Self { path, manifest })
    }

    /// Retrieves a handle over a file in the NPF
    fn open_file(&self, path: &Path) -> Result<NPFFile, NPFExplorationError> {
        let file = File::open(self.path.join(path)).map_err(|err| match err.kind() {
            std::io::ErrorKind::NotFound => {
                NPFExplorationErrorKind::FileNotFound(path.to_path_buf())
            }
            _ => NPFExplorationErrorKind::FileIOError(path.to_path_buf()),
        })?;

        Ok(NPFFile::from(file, PhantomData))
    }

    /// Retrieves the NPF's manifest
    pub fn manifest(&self) -> &Manifest {
        &self.manifest
    }

    /// Retrieves a handle over the NPF's data.tar.gz
    pub fn open_data(&self) -> Result<Option<NPFFile>, NPFExplorationError> {
        self.open_file(Path::new("data.tar.gz")).map_or_else(
            |e| match e.kind() {
                NPFExplorationErrorKind::FileNotFound(_) if self.manifest.kind() != Effective => {
                    Ok(None)
                }
                _ => Err(e),
            },
            |o| Ok(Some(o)),
        )
    }

    /// Retrieves a handle over the NPF's instructions.sh, if one exists
    pub fn open_instructions(&self) -> Result<Option<NPFFile>, NPFExplorationError> {
        self.open_file(Path::new("instructions.sh")).map_or_else(
            |e| match e.kind() {
                NPFExplorationErrorKind::FileNotFound(_) => Ok(None),
                _ => Err(e),
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

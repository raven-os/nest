//! Module to query and manipulate the log files for installed packages

use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use serde_derive::{Deserialize, Serialize};
use tar::EntryType;

/// Enumeration representing the different installable file types
#[derive(Serialize, Deserialize, Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Hash, Debug)]
pub enum FileType {
    /// The file is a directory
    Directory,

    /// The file is a regular file
    File,

    /// The file is a symlink
    Symlink,

    /// The file is a block device
    BlockDevice,

    /// The file is a character device
    CharacterDevice,

    /// The file is a named pipe (also called FIFO)
    FIFO,

    /// The file is a hard link
    Link,
}

impl FileType {
    /// Returns true if the file is a [`Directory`]
    pub fn is_dir(self) -> bool {
        self == FileType::Directory
    }

    /// Returns true if the file is a [`File`]
    pub fn is_file(self) -> bool {
        self == FileType::File
    }

    /// Returns true if the file is a [`Symlink`]
    pub fn is_symlink(self) -> bool {
        self == FileType::Symlink
    }

    /// Returns true if the file is a [`BlockDevice`]
    pub fn is_block_device(self) -> bool {
        self == FileType::BlockDevice
    }

    /// Returns true if the file is a [`CharacterDevice`]
    pub fn is_char_device(self) -> bool {
        self == FileType::CharacterDevice
    }

    /// Returns true if the file is a [`FIFO`]
    pub fn is_fifo(self) -> bool {
        self == FileType::FIFO
    }

    /// Returns true if the file is a [`Link`]
    pub fn is_link(self) -> bool {
        self == FileType::Link
    }
}

impl From<EntryType> for FileType {
    fn from(et: EntryType) -> Self {
        match et {
            EntryType::Directory => FileType::Directory,
            EntryType::Regular => FileType::File,
            EntryType::Symlink => FileType::Symlink,
            EntryType::Block => FileType::BlockDevice,
            EntryType::Char => FileType::CharacterDevice,
            EntryType::Fifo => FileType::FIFO,
            EntryType::Link => FileType::Link,
            _ => unimplemented!(),
        }
    }
}

/// Structure representing a file entry in a log
#[derive(Serialize, Deserialize, Clone, Ord, PartialOrd, PartialEq, Eq, Hash, Debug)]
pub struct FileLogEntry {
    path: PathBuf,
    file_type: FileType,
}

impl FileLogEntry {
    /// Creates a new entry given a path and a file type
    pub fn new(path: PathBuf, file_type: FileType) -> Self {
        FileLogEntry { path, file_type }
    }

    /// Returns a reference over the path for this entry
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns a reference over the file type for this entry
    pub fn file_type(&self) -> &FileType {
        &self.file_type
    }

    /// Returns a mutable reference over the path for this entry
    pub fn path_mut(&mut self) -> &mut PathBuf {
        &mut self.path
    }

    /// Returns a mutable reference over the file type for this entry
    pub fn file_type_mut(&mut self) -> &mut FileType {
        &mut self.file_type
    }
}

/// Structure representing the log for an installed package
#[derive(Serialize, Deserialize, Clone, Ord, PartialOrd, PartialEq, Eq, Hash, Debug)]
pub struct Log {
    files: Vec<FileLogEntry>,
}

impl Log {
    /// Loads a log from a given file
    pub(crate) fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let path = path.as_ref();
        let file = File::open(path)?;
        let log = serde_json::from_reader(&file)?;
        Ok(log)
    }

    /// Saves a log to a given file
    pub(crate) fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), std::io::Error> {
        let path = path.as_ref();
        let mut file = File::create(path)?;
        file.write_all(serde_json::to_string_pretty(&self)?.as_bytes())?;
        file.write_all(&[b'\n'])?;
        Ok(())
    }

    /// Creates a new Log
    pub fn new(files: Vec<FileLogEntry>) -> Self {
        Self { files }
    }

    /// Returns a slice of the file entries in the log
    pub fn files(&self) -> &[FileLogEntry] {
        &self.files
    }
}

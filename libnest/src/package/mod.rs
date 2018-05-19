//! Packages and their content.
//!
//! Packages are made of three things, represented as three different files:
//!  * The manifest: name, version, dependencies, etc.
//!  * The data to install: a compressed tarball (`.tar.gz`).
//!  * The build file: instructions to follow when installing / removing the package. It's taking
//!  the form of a shell script (`.sh`).
//!
//! The first ones (manifests) are downloaded when updating a repository's cache. They are stored on
//! the targeted system.
//!
//! This representation is suitable for pre-installation processes, like searching for a package
//! or resolving the dependecy graph.
//!
//! The other ones are downloaded when installing the package, to avoid filling the user's disk.

mod manifest;
pub use self::manifest::{Manifest, Metadata};

use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;

use config::Config;
use repository::Repository;

/// Represents a package as a whole: its manifest, its data and its build file.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Package<'a> {
    repository: &'a Repository,
    manifest: Manifest,
}

impl<'a> Package<'a> {
    /// Creates a package from its repository and its manifest.
    ///
    /// Usually, you'd like to use a query to get one instead of making it by hand.
    #[inline]
    pub fn from(repository: &'a Repository, manifest: Manifest) -> Package<'a> {
        Package {
            repository,
            manifest,
        }
    }

    /// Returns the repository the package belongs to.
    #[inline]
    pub fn repository(&self) -> &Repository {
        self.repository
    }

    /// Returns the manifest of the package.
    #[inline]
    pub fn manifest(&self) -> &Manifest {
        &self.manifest
    }

    /// Returns the path where data should be located.
    #[inline]
    pub fn data_path(&self, config: &Config) -> PathBuf {
        let mut path = config.download().to_path_buf();
        path.push(self.repository.name());
        path.push(self.manifest.metadata().category());
        path.push(self.manifest.metadata().name());
        path.set_extension("tar.gz");
        path
    }
}

impl<'a> Display for Package<'a> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}::{}/{}",
            self.repository().name(),
            self.manifest.metadata().category(),
            self.manifest.metadata().name(),
        )
    }
}

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
mod name;
pub use self::manifest::{Manifest, Metadata};
pub use self::name::{PackageFullName, PackageId, PackageRequirement};

use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::path::Path;

use failure::{Error, ResultExt};
use serde_json;

use crate::repository::Repository;

/// A handler that encapsulate a manifest and the repository's name this package belongs.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Package {
    repository: String,
    manifest: Manifest,
}

impl Package {
    /// Creates a new package from a [`Repository`] and a [`Manifest`].
    #[inline]
    pub fn from(repository: &Repository, manifest: Manifest) -> Package {
        Package {
            repository: repository.name().to_string(),
            manifest,
        }
    }

    #[inline]
    pub(crate) fn load<P: AsRef<Path>>(repository: String, cache: P) -> Result<Package, Error> {
        let file =
            File::open(cache.as_ref()).with_context(|_| cache.as_ref().display().to_string())?;

        Ok(Package {
            repository,
            manifest: serde_json::from_reader(&file)
                .with_context(|_| cache.as_ref().display().to_string())?,
        })
    }

    /// Returns the repository's name this package belongs to.
    #[inline]
    pub fn repository(&self) -> &str {
        &self.repository
    }

    /// Returns the manifest of the package.
    #[inline]
    pub fn manifest(&self) -> &Manifest {
        &self.manifest
    }

    /// Returns the [`PackageFullName`] of this package, that is a structure representing the `stable::category/package_name` part of the package's id.
    #[inline]
    pub fn full_name(&self) -> PackageFullName {
        PackageFullName::from(
            self.repository.clone(),
            self.manifest.metadata().category().to_string(),
            self.manifest.metadata().name().to_string(),
        )
    }

    /// Returns the [`PackageId`] of this package: a unique identifier to represent a package, in the form `stable::category/package_name#version`.
    #[inline]
    pub fn id(&self) -> PackageId {
        PackageId::from(
            PackageFullName::from(
                self.repository.to_string(),
                self.manifest.metadata().category().to_string(),
                self.manifest.metadata().name().to_string(),
            ),
            self.manifest.metadata().version().clone(),
        )
    }
}

impl Display for Package {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.id())
    }
}

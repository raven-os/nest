use std::collections::HashMap;

use chrono::{DateTime, Utc};
use semver::{Version, VersionReq};
use serde_derive::{Deserialize, Serialize};

use super::Metadata;
use super::{CategoryName, PackageFullName, PackageName, RepositoryName};

/// A manifest that aggregates all versions of a package in one, compact structure.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct PackageManifest {
    name: PackageName,
    category: CategoryName,
    repository: RepositoryName,
    metadata: Metadata,
    kind: Kind,
    versions: HashMap<Version, VersionData>,
}

impl PackageManifest {
    /// Creates a new, empty [`PackageManifest`] from a package, category and repository name.
    ///
    /// Other fields hold their default value.
    pub fn new(
        &self,
        name: PackageName,
        category: CategoryName,
        repository: RepositoryName,
        metadata: Metadata,
    ) -> Self {
        Self {
            name,
            category,
            repository,
            metadata,
            kind: Kind::default(),
            versions: HashMap::new(),
        }
    }

    /// Returns a reference over the name of the package
    #[inline]
    pub fn name(&self) -> &PackageName {
        &self.name
    }

    /// Returns a mutable reference over the name of the package
    #[inline]
    pub fn name_mut(&mut self) -> &mut PackageName {
        &mut self.name
    }

    /// Returns a reference over the category of the package
    #[inline]
    pub fn category(&self) -> &CategoryName {
        &self.category
    }

    /// Returns a mutable reference over the category of the package
    #[inline]
    pub fn category_mut(&mut self) -> &mut CategoryName {
        &mut self.category
    }

    /// Returns a reference over the repository of the package
    #[inline]
    pub fn repository(&self) -> &RepositoryName {
        &self.repository
    }

    /// Returns a mutable reference over the repository of the package
    #[inline]
    pub fn repository_mut(&mut self) -> &mut RepositoryName {
        &mut self.repository
    }

    /// Returns a reference over the metadata of the package
    #[inline]
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    /// Returns a mutable reference over the metadata of the package
    #[inline]
    pub fn metadata_mut(&mut self) -> &mut Metadata {
        &mut self.metadata
    }

    /// Returns the kind of the package
    #[inline]
    pub fn kind(&self) -> Kind {
        self.kind
    }

    /// Returns a mutable reference over the kind of the package
    #[inline]
    pub fn kind_mut(&mut self) -> &mut Kind {
        &mut self.kind
    }

    /// Returns a reference over a [`HashMap`] containing the different versions available for this package, and some
    /// version-dependent data like a list of dependencies.
    #[inline]
    pub fn versions(&self) -> &HashMap<Version, VersionData> {
        &self.versions
    }

    /// Returns a mutable reference over a [`HashMap`] containing the different versions available for this package, and some
    /// version-dependent data like a list of dependencies.
    #[inline]
    pub fn versions_mut(&mut self) -> &mut HashMap<Version, VersionData> {
        &mut self.versions
    }
}

/// A manifest that represent a unique package and its medata.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct Manifest {
    name: PackageName,
    category: CategoryName,
    version: Version,
    metadata: Metadata,
    wrap_date: DateTime<Utc>,
    dependencies: HashMap<PackageFullName, VersionReq>,
}

impl Manifest {
    /// Creates a new, empty [`Manifest`] from a package, category and version name.
    ///
    /// Other fields hold their default value.
    #[inline]
    pub fn new(
        name: PackageName,
        category: CategoryName,
        version: Version,
        metadata: Metadata,
    ) -> Self {
        Self {
            name,
            category,
            version,
            metadata,
            wrap_date: Utc::now(),
            dependencies: HashMap::new(),
        }
    }

    /// Returns a reference over the name of the package
    #[inline]
    pub fn name(&self) -> &PackageName {
        &self.name
    }

    /// Returns a mutable reference over the name of the package
    #[inline]
    pub fn name_mut(&mut self) -> &mut PackageName {
        &mut self.name
    }

    /// Returns a reference over the category of the package
    #[inline]
    pub fn category(&self) -> &CategoryName {
        &self.category
    }

    /// Returns a mutable reference over the category of the package
    #[inline]
    pub fn category_mut(&mut self) -> &mut CategoryName {
        &mut self.category
    }

    /// Returns a reference over the version of the package
    #[inline]
    pub fn version(&self) -> &Version {
        &self.version
    }

    /// Returns a mutable reference over the version of the package
    #[inline]
    pub fn version_mut(&mut self) -> &mut Version {
        &mut self.version
    }

    /// Returns a reference over the metadata of the package
    #[inline]
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    /// Returns a mutable reference over the metadata of the package
    #[inline]
    pub fn metadata_mut(&mut self) -> &mut Metadata {
        &mut self.metadata
    }

    /// Returns a reference over the wrap date of the package
    #[inline]
    pub fn wrap_date(&self) -> &DateTime<Utc> {
        &self.wrap_date
    }

    /// Returns a mutable reference over the wrap date of the package
    #[inline]
    pub fn wrap_date_mut(&mut self) -> &mut DateTime<Utc> {
        &mut self.wrap_date
    }

    /// Returns a reference over the package's dependencies
    #[inline]
    pub fn dependencies(&self) -> &HashMap<PackageFullName, VersionReq> {
        &self.dependencies
    }

    /// Returns a mutable reference over the package's dependencies
    #[inline]
    pub fn dependencies_mut(&mut self) -> &mut HashMap<PackageFullName, VersionReq> {
        &mut self.dependencies
    }
}

/// A container holding that differs from one version to another of the same package.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct VersionData {
    wrap_date: DateTime<Utc>,
    dependencies: HashMap<PackageFullName, VersionReq>,
}

impl VersionData {
    /// Creates a new [`VersionData`] from a wrap date and a list of dependencies.
    #[inline]
    pub fn from(
        wrap_date: DateTime<Utc>,
        dependencies: HashMap<PackageFullName, VersionReq>,
    ) -> Self {
        Self {
            wrap_date,
            dependencies,
        }
    }

    /// Returns a reference over the wrap date of the package
    #[inline]
    pub fn wrap_date(&self) -> &DateTime<Utc> {
        &self.wrap_date
    }

    /// Returns a mutable reference over the wrap date of the package
    #[inline]
    pub fn wrap_date_mut(&mut self) -> &mut DateTime<Utc> {
        &mut self.wrap_date
    }

    /// Returns a reference over the package's dependencies
    #[inline]
    pub fn dependencies(&self) -> &HashMap<PackageFullName, VersionReq> {
        &self.dependencies
    }

    /// Returns a mutable reference over the package's dependencies
    #[inline]
    pub fn dependencies_mut(&mut self) -> &mut HashMap<PackageFullName, VersionReq> {
        &mut self.dependencies
    }
}

/// A package's kind.
///
/// All entities called 'package' may not represent the same thing.
///
/// Some are actual binaries or libraries like one may expect ('effective' packages), but
/// others may be entirely empty, used only to name a list of dependencies ('virtual' packages).
///
/// The `Kind` enum is used to differentiate those packages and speed up their installation process.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum Kind {
    /// The package contains some installable data.
    Effective,
    /// The package doesn't contain any data.
    Virtual,
}

impl Default for Kind {
    fn default() -> Self {
        Kind::Effective
    }
}

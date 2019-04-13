use std::collections::HashMap;

use chrono::{DateTime, Utc};
use semver::{Version, VersionReq};
use serde_derive::{Deserialize, Serialize};

use super::Metadata;
use super::{CategoryName, PackageFullName, PackageName, RepositoryName, PackageID};

/// A manifest that aggregates all versions of a package in one, compact structure.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct PackageManifest {
    name: PackageName,
    category: CategoryName,
    repository: RepositoryName,
    metadata: Metadata,
    versions: HashMap<Version, VersionData>,
}

impl PackageManifest {
    /// Creates a new, empty [`PackageManifest`] from a package, category and repository name.
    ///
    /// Other fields hold their default value.
    pub fn new(
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

    /// Generates the [`PackageFullname`], the common part in the [`PackageID`] of all packages included in this manifest.
    #[inline]
    pub fn full_name(&self) -> PackageFullName {
        PackageFullName::from(
            self.repository().clone(),
            self.category().clone(),
            self.name().clone(),
        )
    }
}

/// A manifest that represent a unique package and its medata.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct Manifest {
    name: PackageName,
    category: CategoryName,
    version: Version,
    kind: Kind,
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
        kind: Kind,
        metadata: Metadata,
    ) -> Self {
        Self {
            name,
            category,
            version,
            kind,
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

    /// Generates the [`PackageID`] of this package given its missing piece: the [`RepositoryName`].
    #[inline]
    pub fn id(&self, repository_name: RepositoryName) -> PackageID {
        PackageID::from(
            self.full_name(repository_name),
            self.version().clone(),
        )
    }

    /// Generates the [`PackageFullName`] of this package given its missing piece: the [`RepositoryName`].
    #[inline]
    pub fn full_name(&self, repository_name: RepositoryName) -> PackageFullName {
        PackageFullName::from(
            repository_name,
            self.category().clone(),
            self.name().clone(),
        )
    }
}

/// A container holding that differs from one version to another of the same package.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct VersionData {
    kind: Kind,
    wrap_date: DateTime<Utc>,
    dependencies: HashMap<PackageFullName, VersionReq>,
}

impl VersionData {
    /// Creates a new [`VersionData`] from a wrap date and a list of dependencies.
    #[inline]
    pub fn from(
        kind: Kind,
        wrap_date: DateTime<Utc>,
        dependencies: HashMap<PackageFullName, VersionReq>,
    ) -> Self {
        Self {
            kind,
            wrap_date,
            dependencies,
        }
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

    #[serde(rename = "effective")]
    /// The package contains some installable data.
    Effective,

    #[serde(rename = "virtual")]
    /// The package doesn't contain any data.
    Virtual,
}

impl Default for Kind {
    fn default() -> Self {
        Kind::Effective
    }
}

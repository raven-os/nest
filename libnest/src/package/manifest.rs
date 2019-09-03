use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::ops::Deref;
use std::path::Path;

use chrono::{DateTime, Utc};
use failure::{Error, ResultExt};
use lazy_static::lazy_static;
use regex::Regex;
use semver::Version;
use serde::de::Visitor;
use serde_derive::{Deserialize, Serialize};

use super::error::SlotParseError;
use super::Metadata;
use super::{
    CategoryName, PackageFullName, PackageID, PackageName, PackageRequirement, PackageShortName,
    RepositoryName,
};

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

    #[inline]
    pub(crate) fn load_from_cache<P: AsRef<Path>>(cache_path: P) -> Result<Self, Error> {
        let file =
            File::open(cache_path.as_ref()).context(cache_path.as_ref().display().to_string())?;

        Ok(serde_json::from_reader(&file).context(cache_path.as_ref().display().to_string())?)
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

    /// Generates the [`PackageShortName`] of this package
    pub fn short_name(&self) -> PackageShortName {
        PackageShortName::from(self.category().clone(), self.name().clone())
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

    /// Regenerates the [`Manifest`] of this [`PackageManifest`] for the given [`Version`].
    pub fn get_manifest_for_version(&self, version: Version) -> Option<Manifest> {
        self.versions.get(&version).map(|version_data| {
            Manifest::new(
                self.name.clone(),
                self.category.clone(),
                version,
                self.metadata.clone(),
                version_data.clone(),
            )
        })
    }

    /// Obtain an iterator over the [`Manifest`] of the available versions of this package
    pub fn iter_manifests<'a>(&'a self) -> impl Iterator<Item = Manifest> + 'a {
        self.versions.iter().map(move |(version, version_data)| {
            Manifest::new(
                self.name.clone(),
                self.category.clone(),
                version.clone(),
                self.metadata.clone(),
                version_data.clone(),
            )
        })
    }
}

/// A manifest that represent a unique package and its metadata.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct Manifest {
    name: PackageName,
    category: CategoryName,
    version: Version,
    metadata: Metadata,
    #[serde(default)]
    slot: Slot,
    #[serde(default)]
    kind: Kind,
    wrap_date: DateTime<Utc>,
    dependencies: HashSet<PackageRequirement>,
}

impl Manifest {
    /// Creates a new, empty [`Manifest`] from a package name, category name, version and [`VersionData`].
    #[inline]
    pub fn new(
        name: PackageName,
        category: CategoryName,
        version: Version,
        metadata: Metadata,
        version_data: VersionData,
    ) -> Self {
        Self {
            name,
            category,
            version,
            metadata,
            slot: version_data.slot,
            kind: version_data.kind,
            wrap_date: version_data.wrap_date,
            dependencies: version_data.dependencies,
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

    /// Returns a reference over the slot of the package
    #[inline]
    pub fn slot(&self) -> &Slot {
        &self.slot
    }

    /// Returns a mutable reference over the slot of the package
    #[inline]
    pub fn slot_mut(&mut self) -> &mut Slot {
        &mut self.slot
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
    pub fn dependencies(&self) -> &HashSet<PackageRequirement> {
        &self.dependencies
    }

    /// Returns a mutable reference over the package's dependencies
    #[inline]
    pub fn dependencies_mut(&mut self) -> &mut HashSet<PackageRequirement> {
        &mut self.dependencies
    }

    /// Generates the [`PackageShortName`] of this package
    pub fn short_name(&self) -> PackageShortName {
        PackageShortName::from(self.category().clone(), self.name().clone())
    }

    /// Generates the [`PackageID`] of this package given its missing piece: the [`RepositoryName`].
    #[inline]
    pub fn id(&self, repository_name: RepositoryName) -> PackageID {
        PackageID::from(
            repository_name,
            self.category().clone(),
            self.name().clone(),
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
    #[serde(default)]
    slot: Slot,
    #[serde(default)]
    kind: Kind,
    wrap_date: DateTime<Utc>,
    dependencies: HashSet<PackageRequirement>,
}

impl VersionData {
    /// Creates a new [`VersionData`] from a wrap date and a list of dependencies.
    #[inline]
    pub fn from(
        slot: Slot,
        kind: Kind,
        wrap_date: DateTime<Utc>,
        dependencies: HashSet<PackageRequirement>,
    ) -> Self {
        Self {
            slot,
            kind,
            wrap_date,
            dependencies,
        }
    }

    /// Returns a reference over the slot of the package
    #[inline]
    pub fn slot(&self) -> &Slot {
        &self.slot
    }

    /// Returns a mutable reference over the slot of the package
    #[inline]
    pub fn slot_mut(&mut self) -> &mut Slot {
        &mut self.slot
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
    pub fn dependencies(&self) -> &HashSet<PackageRequirement> {
        &self.dependencies
    }

    /// Returns a mutable reference over the package's dependencies
    #[inline]
    pub fn dependencies_mut(&mut self) -> &mut HashSet<PackageRequirement> {
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

/// A version's slot.
///
/// A slot is an identifier shared by multiple versions to show that they are not compatible with
/// each other. On the other hand, two versions of the same package with a different slot are compatible
/// and can be installed at the same time.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Slot(String);

impl Slot {
    /// Parses the string representation of a [`Slot`].
    pub fn parse(repr: &str) -> Result<Self, SlotParseError> {
        Self::try_from(repr)
    }
}

impl Display for Slot {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

impl Deref for Slot {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Slot {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Default for Slot {
    fn default() -> Self {
        Self(String::new())
    }
}

impl TryFrom<&str> for Slot {
    type Error = SlotParseError;

    #[inline]
    fn try_from(repr: &str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref SLOT_REGEX: Regex = Regex::new(r"^[a-z0-9\.\-]*$").unwrap();
        }

        if SLOT_REGEX.is_match(repr) {
            Ok(Self(String::from(repr)))
        } else {
            Err(SlotParseError(repr.to_string()))
        }
    }
}

struct SlotVisitor;

impl<'de> Visitor<'de> for SlotVisitor {
    type Value = Slot;

    #[inline]
    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str("a slot")
    }

    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Slot::try_from(value).map_err(|_| E::custom("the slot value isn't valid"))
    }
}

impl_serde_visitor!(Slot, SlotVisitor);

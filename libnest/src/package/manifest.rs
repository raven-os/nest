//! A Manifest: a record of all metadata, dependencies, etc. of a package.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use semver::{Version, VersionReq};
use serde_derive::{Deserialize, Serialize};

use crate::package::PackageFullName;

/// A subpart of the manifest.
///
/// They represent a package's name, category, description, etc.
///
/// All primitive informations that may be relevant when looking for packages.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Metadata {
    name: String,
    category: String,
    version: Version,
    #[serde(default)]
    description: String,
    #[serde(default)]
    tags: String,
    created_at: DateTime<Utc>,
}

impl Metadata {
    /// Returns the name of the package.
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the category of the package.
    #[inline]
    pub fn category(&self) -> &str {
        &self.category
    }

    /// Returns the version of the package
    #[inline]
    pub fn version(&self) -> &Version {
        &self.version
    }

    /// Returns the description of the package
    #[inline]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the tags of the package
    #[inline]
    pub fn tags(&self) -> &str {
        &self.tags
    }

    /// Returns the creation date of the package
    #[inline]
    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
}

/// A package's metadata, dependencies, etc.
///
/// All these informations are obtained when the repository which this package belongs to is pulled. Therefore, they
/// may be out of date.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct Manifest {
    metadata: Metadata,
    #[serde(default)]
    dependencies: HashMap<PackageFullName, VersionReq>,
}

impl Manifest {
    /// Returns the package's metadata, like its name, version, category, etc.
    #[inline]
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    /// Returns the package's dependencies, like it's name, version, category etc.
    #[inline]
    pub fn dependencies(&self) -> &HashMap<PackageFullName, VersionReq> {
        &self.dependencies
    }
}

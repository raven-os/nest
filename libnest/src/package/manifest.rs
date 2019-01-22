//! Provides `PackageManifest`, a type representing a package's metadata and dependencies.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use semver::{Version, VersionReq};
use serde_derive::{Deserialize, Serialize};

use super::identification::PackageFullName;
use super::{CategoryName, Kind, PackageName};

/// Represents a package's metadata, like its name, category, description, etc.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Metadata {
    name: PackageName,
    category: CategoryName,
    version: Version,
    #[serde(default)]
    description: String,
    #[serde(default)]
    tags: String,
    created_at: DateTime<Utc>,
}

impl Metadata {
    /// Returns the name of the package
    #[inline]
    pub fn name(&self) -> &PackageName {
        &self.name
    }

    /// Returns the category of the package
    #[inline]
    pub fn category(&self) -> &CategoryName {
        &self.category
    }

    /// Returns the version of the package
    #[inline]
    pub fn version(&self) -> &Version {
        &self.version
    }

    /// Returns the description of the package
    #[inline]
    pub fn description(&self) -> &String {
        &self.description
    }

    /// Returns the tags of the package
    #[inline]
    pub fn tags(&self) -> &str {
        &self.tags
    }
}

/// Represents a package's manifest. It wraps the package's metadata and the available
/// versions of this package for the current architecture.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Manifest {
    metadata: Metadata,
    #[serde(default)]
    kind: Kind,
    #[serde(default)]
    dependencies: HashMap<PackageFullName, VersionReq>,
}

impl Manifest {
    /// Returns the package's metadata
    #[inline]
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    /// Returns the package's kind
    #[inline]
    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    /// Returns the package's dependencies
    #[inline]
    pub fn dependencies(&self) -> &HashMap<PackageFullName, VersionReq> {
        &self.dependencies
    }
}

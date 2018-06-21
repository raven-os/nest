//! A Manifest: a record of all metadata, dependencies etc. of a package.

use std::collections::HashMap;

use semver::{Version, VersionReq};

use package::PackageFullName;

/// A subpart of the manifest.
///
/// They represent a package's name, category, description etc.
///
/// All primitives informations that may be relevant when looking for packages.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Metadata {
    name: String,
    category: String,
    version: Version,
}

impl Metadata {
    /// Returns the name of the package
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the category of the package
    #[inline]
    pub fn category(&self) -> &str {
        &self.category
    }

    /// Returns the version of the package
    #[inline]
    pub fn version(&self) -> &Version {
        &self.version
    }
}

/// A package's metadata, dependencies etc.
///
/// All these informations are got when the repository which this package belongs to is pulled. Therefore, they
/// may be out of date.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct Manifest {
    metadata: Metadata,
    dependencies: HashMap<PackageFullName, VersionReq>,
}

impl Manifest {
    /// Returns the package's metadata, like it's name, version, category etc.
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

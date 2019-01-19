//! Package identification

use crate::package::{CategoryName, PackageName, RepositoryName};
use semver::Version;

/// Full name of a package, which is the combination of its repository, category and name
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct PackageFullName {
    repository: RepositoryName,
    category: CategoryName,
    name: PackageName,
}

impl PackageFullName {
    /// Creates a [`PackageFullName`] from a [`RepositoryName`], a [`CategoryName`] and a [`PackageName`]
    #[inline]
    pub fn from(repository: RepositoryName, category: CategoryName, name: PackageName) -> Self {
        PackageFullName {
            repository,
            category,
            name,
        }
    }

    /// Returns a reference over the repository name
    #[inline]
    pub fn repository(&self) -> &RepositoryName {
        &self.repository
    }

    /// Returns a reference over the category name
    #[inline]
    pub fn category(&self) -> &CategoryName {
        &self.category
    }

    /// Returns a reference over the package name
    #[inline]
    pub fn name(&self) -> &PackageName {
        &self.name
    }
}

impl std::fmt::Display for PackageFullName {
    #[inline]
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            fmt,
            "{}::{}/{}",
            self.repository(),
            self.category(),
            self.name()
        )
    }
}

/// Identity of a package, which is the combination of its full name and its version
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct PackageID {
    full_name: PackageFullName,
    version: Version,
}

impl PackageID {
    /// Creates a [`PackageID`] from a [`PackageFullName`] and a [`Version`]
    #[inline]
    pub fn from(full_name: PackageFullName, version: Version) -> Self {
        PackageID { full_name, version }
    }

    /// Returns a reference over the package's full name
    #[inline]
    pub fn full_name(&self) -> &PackageFullName {
        &self.full_name
    }

    /// Returns a reference over the package's version
    #[inline]
    pub fn version(&self) -> &Version {
        &self.version
    }
}

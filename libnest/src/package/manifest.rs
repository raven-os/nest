//! Provides `PackageManifest`, a type representing a package's metadata and dependencies.

use crate::package::{CategoryName, Kind, PackageName, RepositoryName, Tag};
use semver::Version;

/// Represents a package's metadata, like its name, category, description etc.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Metadata {
    name: PackageName,
    category: CategoryName,
    repository: RepositoryName,
    description: String,
    tags: Vec<Tag>,
}

/// Represents a package's manifest. It wraps the package's metadata and the available
/// versions of this package for the current architecture.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Manifest {
    metadata: Metadata,
    kind: Kind,
    versions: Vec<Version>,
}

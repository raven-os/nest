//! A Manifest: a record of all metadata, dependencies, etc. of a package.

/// A subpart of the manifest.
///
/// They represent a package's name, category, description, etc.
///
/// All primitive informations that may be relevant when looking for packages.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Metadata {
    name: String,
    category: String,
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
}

/// A package's metadata, dependencies, etc.
///
/// All these informations are obtained when the repository which this package belongs to is pulled. Therefore, they
/// may be out of date.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Manifest {
    metadata: Metadata,
}

impl Manifest {
    /// Returns the package's metadata, like its name, version, category, etc.
    #[inline]
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

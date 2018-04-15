//! A Manifest: a record of all metadatas, dependencies etc. of a package.

/// A subpart of the manifest.
///
/// They represent a package's name, category, description etc.
///
/// All primitives informations that may be relevant when search packages.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Metadatas {
    name: String,
    category: String,
}

impl Metadatas {
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
}

/// A package's metadatas, dependencies etc.
///
/// All these informations are got when the repository which this package belongs to is pulled. Therefore, they
/// may be out of date.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Manifest {
    metadatas: Metadatas,
}

impl Manifest {
    /// Returns the package's metadatas, like it's name, version, category etc.
    #[inline]
    pub fn metadatas(&self) -> &Metadatas {
        &self.metadatas
    }
}

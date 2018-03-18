//! Packages representation and their metadatas.
//!
//! Packages represented in this module are **local** packages, as represented in the local cache.
//! They are not updated until the `pull` operation is done.
//!
//! They **only** represent their metadatas (and not their content) and may not be installed.
//!
//! This representation is suitable for pre-installation processes, like searching for a package
//! or resolving the dependecy graph.

/// A package and it's meta-datas, like it's name, version, category etc.
///
/// All these informations are get when pulling the repository this package belongs to, and therefore may
/// be out of date.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Package {
    name: String,
    category: String,
}

impl Package {
    /// Returns the name of the package
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the category of the package
    pub fn category(&self) -> &str {
        &self.category
    }
}

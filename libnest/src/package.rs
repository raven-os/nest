//! Package

/// Represents a package and it's meta-datas, like it's name, version, category etc.
///
/// All these informations are get when pulling the repository this package belongs to, and therefore may
/// be out of date.
#[derive(Serialize, Deserialize)]
#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
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

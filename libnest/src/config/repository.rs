use url_serde::SerdeUrl;

/// A structure holding all the configuration of a repository: mirrors, proxy etc.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct RepositoryConfig {
    mirrors: Vec<SerdeUrl>,
}

impl RepositoryConfig {
    /// Creates a new, empty [`RepositoryConfig`].
    #[inline]
    pub fn new() -> RepositoryConfig {
        RepositoryConfig {
            mirrors: Vec::new(),
        }
    }

    /// Returns a reference over a vector of [`SerdeUrl`], which are the mirrors of this repository.
    /// They are sorted by order of importance: the first one should be used in priority etc.
    #[inline]
    pub fn mirrors(&self) -> &Vec<SerdeUrl> {
        &self.mirrors
    }

    /// Returns a mutable reference over a vector of [`SerdeUrl`], which are the mirrors of this repository.
    /// They should be kept sorted by order of importance.
    #[inline]
    pub fn mirrors_mut(&mut self) -> &mut Vec<SerdeUrl> {
        &mut self.mirrors
    }
}

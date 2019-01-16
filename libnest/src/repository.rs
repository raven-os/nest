//! Repository: wrapper around a name and a [`RepositoryConfig`]

use crate::config::RepositoryConfig;

/// A repository
///
/// Wraps a reference over a name and a repository configuration
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Repository<'a, 'b> {
    name: &'a str,
    config: &'b RepositoryConfig,
}

impl<'a, 'b> Repository<'a, 'b> {
    pub(crate) fn from(name: &'a str, config: &'b RepositoryConfig) -> Repository<'a, 'b> {
        Repository { name, config }
    }

    /// Returns the name of the repository
    pub fn name(&self) -> &str {
        self.name
    }

    /// Returns the configuration of the repository
    pub fn config(&self) -> &RepositoryConfig {
        self.config
    }
}

//! Repositories: wrappers around a name and a [`RepositoryConfig`].

use crate::config::{Config, RepositoryConfig};
use crate::transaction::Transfer;

/// A repository.
///
/// It's wraps a reference over a name, and a configuration (which, in turns, holds a list of mirror etc.).
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Repository<'a, 'b> {
    name: &'a str,
    config: &'b RepositoryConfig,
}

impl<'a, 'b> Repository<'a, 'b> {
    pub(crate) fn from(name: &'a str, config: &'b RepositoryConfig) -> Repository<'a, 'b> {
        Repository { name, config }
    }

    /// Returns the name of the repository.
    pub fn name(&self) -> &str {
        self.name
    }

    /// Returns the configuration of the repository.
    pub fn config(&self) -> &RepositoryConfig {
        self.config
    }

    /// Returns a new [`Transfer`] to download a target file from this repository.
    pub fn transfer<'c>(&self, config: &'c Config) -> Transfer<'c, 'b> {
        Transfer::from(config, self.config)
    }
}

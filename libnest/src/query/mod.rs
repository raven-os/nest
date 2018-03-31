//! Queries to search between installed or available packages.
//!
//! This module provides different queries to look for packages between the installed or availalbe
//! packages.

mod cache;

use std::fmt::{self, Display, Formatter};

pub use self::cache::CacheQuery;

use package::Package;
use repository::Repository;

/// The result of a query.
///
/// Basically, this is a wrapper around a package and it's repository.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct QueriedPackage<'a> {
    repository: &'a Repository,
    content: Package,
}

impl<'a> QueriedPackage<'a> {
    pub(crate) fn from(repository: &'a Repository, content: Package) -> QueriedPackage<'a> {
        QueriedPackage {
            repository,
            content,
        }
    }

    /// Returns the repository where the package was found.
    pub fn repository(&self) -> &Repository {
        self.repository
    }

    /// Returns the cache of the package.
    pub fn content(&self) -> &Package {
        &self.content
    }
}

impl<'a> Display for QueriedPackage<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}::{}/{}",
            self.repository().name(),
            self.content().category(),
            self.content().name(),
        )
    }
}

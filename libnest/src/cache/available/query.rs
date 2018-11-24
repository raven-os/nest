use std::fs;
use std::path::Path;

use failure::{Error, ResultExt};
use semver::Version;

use crate::package::{Package, PackageRequirement};

/// The strategy to use when looking for packages in this cache.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum AvailablePackagesCacheQueryStrategy {
    /// Returns a unique version of each packages matching the given
    /// requirements, the most recent one.
    BestMatch,
    /// Returns all the packages matching the requirements.
    AllMatchesUnsorted,
    /// Returns all the packages matching the requirements, sorted by version,
    /// the most recent ones being the first ones.
    AllMatchesSorted,
}

/// A query on the [`AvailablePackages`][1] cache.
///
/// This handle takes a requirement and a strategy and will look for all packages matching
/// the given requirement following the given strategy.
///
/// [1]: struct.AvailablePackages.html
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct AvailablePackagesCacheQuery<'a, 'b> {
    base: &'a Path,
    requirement: &'b PackageRequirement,
    strategy: AvailablePackagesCacheQueryStrategy,
}

impl<'a, 'b> AvailablePackagesCacheQuery<'a, 'b> {
    #[inline]
    pub(crate) fn from(
        base: &'a Path,
        requirement: &'b PackageRequirement,
    ) -> AvailablePackagesCacheQuery<'a, 'b> {
        AvailablePackagesCacheQuery {
            base,
            requirement,
            strategy: AvailablePackagesCacheQueryStrategy::BestMatch,
        }
    }

    /// Sets the strategy to use.
    #[inline]
    pub fn set_strategy(mut self, strategy: AvailablePackagesCacheQueryStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    fn get_cache_entries(path: &Path) -> Result<impl Iterator<Item = String>, Error> {
        let mut results = Vec::new();

        if path.exists() {
            for entry in fs::read_dir(path).with_context(|_| path.display().to_string())? {
                let entry = entry.with_context(|_| path.display().to_string())?;
                if let Ok(name) = entry.file_name().into_string() {
                    results.push(name);
                }
            }
        }
        Ok(results.into_iter())
    }

    /// Performs the query, returning the results (a [`Vec`][1]<[`Package`][2]>) in case of success.
    ///
    /// [1]: https://doc.rust-lang.org/std/vec/struct.Vec.html
    /// [2]: ../../package/struct.Package.html
    pub fn perform(&self) -> Result<Vec<Package>, Error> {
        let mut results = Vec::new();

        // Get all repositories and filter-out the bad ones
        let repositories = Self::get_cache_entries(self.base)?.filter(|repository| {
            match self.requirement.repository() {
                Some(requirement) => repository == requirement,
                _ => true,
            }
        });
        for repository in repositories {
            let path = self.base.join(&repository);

            // Get all categories and filter-out the bad ones
            let categories = Self::get_cache_entries(&path)?.filter(|category| {
                match self.requirement.category() {
                    Some(requirement) => category == requirement,
                    _ => true,
                }
            });
            for category in categories {
                let path = path.join(category);

                // Get all packages and filter-out the bad ones
                let packages = Self::get_cache_entries(&path)?
                    .filter(|package| self.requirement.name() == package);
                for package in packages {
                    let path = path.join(package);

                    // Get all versions
                    let mut versions = Self::get_cache_entries(&path)?
                        .map(|ver_str| Version::parse(&ver_str))
                        .collect::<Result<Vec<_>, _>>()?;

                    // Apply the chosen stragegy
                    match self.strategy {
                        AvailablePackagesCacheQueryStrategy::BestMatch => {
                            versions.sort_unstable_by(|a, b| b.cmp(a));
                            let result = versions
                                .iter()
                                .find(|version| self.requirement.version_req().matches(version));
                            if let Some(version) = result {
                                let path = path.join(version.to_string());
                                results.push(Package::load(repository.clone(), &path)?);
                            }
                        }
                        AvailablePackagesCacheQueryStrategy::AllMatchesUnsorted => {
                            results.append(
                                &mut versions
                                    .iter()
                                    .filter(|version| {
                                        self.requirement.version_req().matches(version)
                                    })
                                    .map(|version| {
                                        Package::load(
                                            repository.clone(),
                                            &path.join(version.to_string()),
                                        )
                                    })
                                    .collect::<Result<Vec<_>, _>>()?,
                            );
                        }
                        AvailablePackagesCacheQueryStrategy::AllMatchesSorted => {
                            versions.sort_unstable_by(|a, b| b.cmp(a));
                            results.append(
                                &mut versions
                                    .iter()
                                    .filter(|version| {
                                        self.requirement.version_req().matches(version)
                                    })
                                    .map(|version| {
                                        Package::load(
                                            repository.clone(),
                                            &path.join(version.to_string()),
                                        )
                                    })
                                    .collect::<Result<Vec<_>, _>>()?,
                            );
                        }
                    }
                }
            }
        }
        Ok(results)
    }
}

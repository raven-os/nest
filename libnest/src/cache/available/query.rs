use std::fs;
use std::path::Path;

use failure::{Error, ResultExt};
use semver::Version;

use crate::package::{CategoryName, Package, PackageRequirement, RepositoryName};

/// The strategy to use when looking for packages in this cache.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum AvailablePackagesCacheQueryStrategy {
    /// This strategy can be used to obtain only the most recent version of each package
    BestMatch,

    /// This strategy can be used to obtain all the packages matching the requirements, unsorted
    AllMatchesUnsorted,

    /// This strategy can be used to obtain all the packages matching the requirements, sorted by version,
    /// with the most recent first.
    AllMatchesSorted,
}

/// Structure representing a query in the [`AvailablePackages`] cache.
///
/// It can be constructed from a [`PackageRequirement`] and a strategy and will look for all
/// the packages matching the given requirement, following the given strategy.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct AvailablePackagesCacheQuery<'a, 'b> {
    cache_root: &'a Path,
    requirement: &'b PackageRequirement,
    strategy: AvailablePackagesCacheQueryStrategy,
}

impl<'a, 'b> AvailablePackagesCacheQuery<'a, 'b> {
    #[inline]
    pub(crate) fn from(
        cache_root: &'a Path,
        requirement: &'b PackageRequirement,
    ) -> AvailablePackagesCacheQuery<'a, 'b> {
        AvailablePackagesCacheQuery {
            cache_root,
            requirement,
            strategy: AvailablePackagesCacheQueryStrategy::BestMatch,
        }
    }

    /// Sets the strategy to use when selecting or sorting packages
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

    /// Perform the query
    pub fn perform(&self) -> Result<Vec<Package>, Error> {
        let mut results = Vec::new();

        let repositories = Self::get_cache_entries(&self.cache_root)?
            .filter(|repo| match self.requirement.repository() {
                Some(required_repo) => required_repo.as_str() == repo,
                _ => true,
            })
            .map(RepositoryName::from);

        for repo in repositories {
            let repo_cache_path = self.cache_root.join(&repo);

            let categories = Self::get_cache_entries(&repo_cache_path)?
                .filter(|category| match self.requirement.category() {
                    Some(required_category) => required_category.as_str() == category,
                    _ => true,
                })
                .map(CategoryName::from);

            for category in categories {
                let category_cache_path = repo_cache_path.join(category);

                // TODO: at the moment, we match the package name exactly. This should be configurable.
                let packages = Self::get_cache_entries(&category_cache_path)?
                    .filter(|package_name| self.requirement.name().as_str() == package_name);

                for package in packages {
                    let package_cache_path = category_cache_path.join(package);

                    let mut versions = Self::get_cache_entries(&package_cache_path)?
                        .map(|ver_repr| Version::parse(&ver_repr))
                        .collect::<Result<Vec<_>, _>>()?;

                    match self.strategy {
                        AvailablePackagesCacheQueryStrategy::BestMatch => {
                            versions.sort_unstable_by(|a, b| b.cmp(a));
                            let result = versions.iter().find(|version| {
                                self.requirement.version_requirement().matches(version)
                            });
                            if let Some(version) = result {
                                let path = package_cache_path.join(version.to_string());
                                results.push(Package::load_from_cache(repo.clone(), &path)?);
                            }
                        }
                        AvailablePackagesCacheQueryStrategy::AllMatchesSorted => {
                            versions.sort_unstable_by(|a, b| b.cmp(a));
                            results.append(
                                &mut versions
                                    .iter()
                                    .filter(|version| {
                                        self.requirement.version_requirement().matches(&version)
                                    })
                                    .map(|version| {
                                        Package::load_from_cache(
                                            repo.clone(),
                                            &package_cache_path.join(version.to_string()),
                                        )
                                    })
                                    .collect::<Result<Vec<_>, _>>()?,
                            );
                        }
                        AvailablePackagesCacheQueryStrategy::AllMatchesUnsorted => {
                            results.append(
                                &mut versions
                                    .iter()
                                    .filter(|version| {
                                        self.requirement.version_requirement().matches(&version)
                                    })
                                    .map(|version| {
                                        Package::load_from_cache(
                                            repo.clone(),
                                            &package_cache_path.join(version.to_string()),
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

use std::collections::HashMap;
use std::fs;
use std::iter::FromIterator;
use std::path::Path;

use failure::{Error, ResultExt};

use crate::config::Config;
use crate::package::{
    CategoryName, Manifest, PackageFullName, PackageID, PackageManifest, RepositoryName,
    SoftPackageRequirement,
};

/// The result of a query to the packages cache
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct QueryResult {
    repository: RepositoryName,
    manifest: Manifest,
}

impl QueryResult {
    /// Creates a [`QueryResult`] from a [`RepositoryName`] and a [`Manifest`]
    pub fn from(repository: RepositoryName, manifest: Manifest) -> Self {
        Self {
            repository,
            manifest,
        }
    }

    /// Returns a reference over the repository for this result
    pub fn repository(&self) -> &RepositoryName {
        &self.repository
    }

    /// Returns a mutable reference over the repository for this result
    pub fn repository_mut(&mut self) -> &mut RepositoryName {
        &mut self.repository
    }

    /// Returns a reference over the manifest for this result
    pub fn manifest(&self) -> &Manifest {
        &self.manifest
    }

    /// Returns a mutable reference over the manifest for the package associated with this result
    pub fn manifest_mut(&mut self) -> &mut Manifest {
        &mut self.manifest
    }

    /// Generates the [`PackageFullName`] for this result
    pub fn full_name(&self) -> PackageFullName {
        self.manifest().full_name(self.repository().clone())
    }

    /// Generates the [`PackageID`] for the package associated with this result
    pub fn id(&self) -> PackageID {
        self.manifest().id(self.repository().clone())
    }
}

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
    requirement: &'b SoftPackageRequirement,
    strategy: AvailablePackagesCacheQueryStrategy,
}

impl<'a, 'b> AvailablePackagesCacheQuery<'a, 'b> {
    #[inline]
    pub(crate) fn from(
        cache_root: &'a Path,
        requirement: &'b SoftPackageRequirement,
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
    pub fn perform(&self) -> Result<Vec<QueryResult>, Error> {
        let mut results = Vec::new();

        let repositories = Self::get_cache_entries(&self.cache_root)?
            .filter(|repo| match self.requirement.repository() {
                Some(required_repo) => required_repo.as_str() == repo,
                _ => true,
            })
            .map(|name| {
                RepositoryName::parse(&name).expect("invalid repository name found in the cache")
            });

        for repo in repositories {
            let repo_cache_path = self.cache_root.join(repo.as_str());

            let categories = Self::get_cache_entries(&repo_cache_path)?
                .filter(|category| match self.requirement.category() {
                    Some(required_category) => required_category.as_str() == category,
                    _ => true,
                })
                .map(|name| {
                    CategoryName::parse(&name).expect("invalid category name found in the cache")
                });

            for category in categories {
                let category_cache_path = repo_cache_path.join(category.as_str());

                // TODO: at the moment, we match the package name exactly. This should be configurable.
                let packages = Self::get_cache_entries(&category_cache_path)?
                    .filter(|package_name| self.requirement.name().as_str() == package_name);

                for package in packages {
                    let package_cache_path = category_cache_path.join(package);
                    let package_manifest = PackageManifest::load_from_cache(package_cache_path)?;
                    let mut versions = package_manifest.versions().keys().collect::<Vec<_>>();

                    match self.strategy {
                        AvailablePackagesCacheQueryStrategy::BestMatch => {
                            versions.sort_unstable_by(|a, b| b.cmp(a));
                            let result = versions.iter().find(|version| {
                                self.requirement.version_requirement().matches(version)
                            });
                            if let Some(version) = result {
                                // FIXME: having to ask for a version that we already know exists is meh
                                results.push(QueryResult::from(
                                    repo.clone(),
                                    package_manifest
                                        .get_manifest_for_version((*version).clone())
                                        .unwrap(),
                                ));
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
                                        QueryResult::from(
                                            repo.clone(),
                                            package_manifest
                                                .get_manifest_for_version((*version).clone())
                                                .unwrap(),
                                        )
                                    })
                                    .collect::<Vec<_>>(),
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
                                        QueryResult::from(
                                            repo.clone(),
                                            package_manifest
                                                .get_manifest_for_version((*version).clone())
                                                .unwrap(),
                                        )
                                    })
                                    .collect::<Vec<_>>(),
                            );
                        }
                    }
                }
            }
        }

        Ok(results)
    }

    /// Perform the query, and sort the repositories in order of preference
    pub fn perform_and_sort_by_preference(
        &self,
        config: &Config,
    ) -> Result<Vec<QueryResult>, Error> {
        let map: HashMap<&RepositoryName, usize> = HashMap::from_iter(
            config
                .repositories_order()
                .iter()
                .enumerate()
                .map(|(a, b)| (b, a)),
        );

        self.perform().map(|mut results| {
            results.sort_by(|a, b| map[a.repository()].cmp(&map[b.repository()]));
            results
        })
    }
}

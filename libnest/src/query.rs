//! Queries to search in available packages.
//!
//! This module provides a structure to look for manifests that are stored in the local cache.

use failure::Error;

use config::Config;
use package::Package;
use repository::CategoryCache;
use repository::Repository;

/// A query to search through the local cache of available manifests.
///
/// Remember that local cache is updated only when pulling the corresponding
/// repository.
///
/// # Examples
///
/// ```no_run
/// # extern crate libnest;
/// extern crate url;
///
/// use url::Url;
/// use libnest::config::Config;
/// use libnest::repository::{Repository, Mirror};
/// use libnest::query::CacheQuery;
///
/// # fn func() -> Result<(), url::ParseError> {
/// // Let's setup a basic configuration
/// let mut config = Config::new();
/// let mut repo = Repository::new("stable");
/// let mirror = Mirror::new(Url::parse("http://stable.raven-os.org")?);
///
/// repo.mirrors_mut().push(mirror);
/// config.repositories_mut().push(repo);
///
/// // Let's look for `stable::shell/dash`:
/// let mut query = CacheQuery::new(&config);
/// query.with_repository("stable");
/// query.with_category("shell");
/// query.with_name("dash");
///
/// // Analyze result
/// match query.perform() {
///     Ok(manifests) => println!("{} result(s) found", manifests.len()),
///     Err(e) => eprintln!("error: {}", e),
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct CacheQuery<'a> {
    config: &'a Config,
    name: Option<String>,
    category: Option<String>,
    repository: Option<String>,
}

impl<'a> CacheQuery<'a> {
    /// Creates a new, empty query.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
    /// use libnest::config::Config;
    /// use libnest::query::CacheQuery;
    ///
    /// let config = Config::new();
    /// let mut query = CacheQuery::new(&config);
    /// ```
    #[inline]
    pub fn new(config: &'a Config) -> CacheQuery<'a> {
        CacheQuery {
            config,
            name: None,
            category: None,
            repository: None,
        }
    }

    /// Sets the name of the package to look for.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
    /// use libnest::config::Config;
    /// use libnest::query::CacheQuery;
    ///
    /// let config = Config::new();
    /// let mut query = CacheQuery::new(&config);
    ///
    /// // Will look for all packages named "gcc"
    /// query.with_name("gcc");
    /// ```
    #[inline]
    pub fn with_name(&mut self, name: &str) {
        self.name = Some(name.to_string());
    }

    /// Sets the name of the category the package must be in.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
    /// use libnest::config::Config;
    /// use libnest::query::CacheQuery;
    ///
    /// let config = Config::new();
    /// let mut query = CacheQuery::new(&config);
    ///
    /// // Will look for all packages in the category "sys-devel"
    /// query.with_category("sys-devel");
    /// ```
    #[inline]
    pub fn with_category(&mut self, category: &str) {
        self.category = Some(category.to_string());
    }

    /// Sets the name of the repository the package must be in.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
    /// use libnest::config::Config;
    /// use libnest::query::CacheQuery;
    ///
    /// let config = Config::new();
    /// let mut query = CacheQuery::new(&config);
    ///
    /// // Will look for all packages in the repository "stable".
    /// query.with_repository("stable");
    /// ```
    #[inline]
    pub fn with_repository(&mut self, repository: &str) {
        self.repository = Some(repository.to_string());
    }

    /// Performs the search with the given critera.
    ///
    /// The search may succeed, giving a (possibly empty) vector of
    /// [`Package`](struct.Package.html)s, or fail, mostly for IO reasons.
    ///
    /// # Filesystem
    ///
    /// This operation assumes the current user has the rights to read the local cache.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate libnest;
    /// # use std::error;
    /// # fn test() -> Result<(), Box<error::Error>> {
    /// use libnest::config::Config;
    /// use libnest::query::CacheQuery;
    ///
    /// let config = Config::new();
    /// let mut query = CacheQuery::new(&config);
    ///
    /// // Will look for all packages named `gcc` in the repository `stable`.
    /// query.with_repository("stable");
    /// query.with_name("gcc");
    /// let results = query.perform()?;
    ///
    /// println!("There is {} packages named `gcc` in the repository `stable`.", results.len());
    /// # Ok(())
    /// # }
    /// ```
    // XXX: Improve search performances when repository/category is known.
    // Or maybe with a better way of caching stuff?
    pub fn perform(&self) -> Result<Vec<Package<'a>>, Error> {
        let mut vec: Vec<Package> = Vec::new();

        let repositories: Vec<&Repository> = self
            .config
            .repositories()
            .iter()
            .filter(|repo| {
                if let Some(ref repo_name) = self.repository {
                    repo_name == repo.name()
                } else {
                    true
                }
            })
            .collect();
        for repo in repositories {
            let categories: Vec<CategoryCache> = repo
                .cache(self.config)
                .categories()?
                .filter(|&(ref name, _)| {
                    if let Some(ref cat_name) = self.category {
                        name == cat_name
                    } else {
                        true
                    }
                })
                .map(|(_, cache)| cache)
                .collect();
            for category in categories {
                vec.append(
                    &mut category
                        .manifests()?
                        .filter(|cache| {
                            if let Some(ref package_name) = self.name {
                                cache.manifest().metadata().name() == package_name
                            } else {
                                true
                            }
                        })
                        .map(|cache| Package::from(repo, cache.manifest().clone()))
                        .collect(),
                );
            }
        }
        Ok(vec)
    }
}

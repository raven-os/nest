use std::io;

use config::Config;
use repository::{CategoryCache, Repository};
use query::QueriedPackage;

/// A query to search through the local cache of available packages.
///
/// Remember that local cache is updated only when pulling the corresponding
/// repository.
///
/// # Examples
///
/// ```no_run
/// # extern crate libnest;
/// use libnest::config::Config;
/// use libnest::repository::{Repository, Mirror};
/// use libnest::query::CacheQuery;
///
/// // Let's setup a basic configuration
/// let mut config = Config::new();
/// let mut repo = Repository::new(&config, "stable");
/// let mirror = Mirror::new("http://example.com");
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
///     Ok(packages) => println!("{} result(s) found", packages.len()),
///     Err(e) => eprintln!("error: {}", e),
/// }
/// ```
#[derive(Debug)]
pub struct CacheQuery<'a> {
    config: &'a Config,
    name: Option<String>,
    category: Option<String>,
    repository: Option<String>,
}

impl<'a> CacheQuery<'a> {
    /// Creates a new query, with empty parameters.
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
    pub fn with_repository(&mut self, repository: &str) {
        self.repository = Some(repository.to_string());
    }

    /// Performs the search with the given critera.
    ///
    /// The search may success, giving a (possibly empty) vector of
    /// [`QueriedPackage`](struct.QueriedPackage.html), or fail, mostly for IO reasons.
    ///
    /// # Filesystem
    ///
    /// This operation assumes the current user has the rights to read the local cache.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate libnest;
    /// # use std::io;
    /// # fn test() -> Result<(), io::Error> {
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
    // Or maybe search for a better way of caching stuff?
    pub fn perform(&self) -> Result<Vec<QueriedPackage<'a>>, io::Error> {
        let mut vec: Vec<QueriedPackage> = Vec::new();

        let repositories: Vec<&Repository> = self.config
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
            let categories: Vec<CategoryCache> = repo.cache()
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
                vec.append(&mut category
                    .packages()?
                    .filter(|package| {
                        if let Some(ref package_name) = self.name {
                            package.content().name() == package_name
                        } else {
                            true
                        }
                    })
                    .map(|package| QueriedPackage::from(repo, package.content().clone()))
                    .collect());
            }
        }
        Ok(vec)
    }
}

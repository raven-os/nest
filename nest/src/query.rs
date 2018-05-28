//! Functions to resolve package's name.

use failure::Error;
use libnest::config::Config;
use libnest::package::Package;
use libnest::query::CacheQuery;
use regex::Regex;

use error::QueryErrorKind;

lazy_static! {
    static ref REGEX_PACKAGE_QUERY: Regex = Regex::new(
        r"^((?P<repository>[a-z]+)::)?((?P<category>[a-z\-]+)/)?(?P<package>([a-z_]+))$"
    ).unwrap();
}

/// Looks for the cache of the given argument, whether it's a package, a category or a repository.
pub fn cache<'a>(config: &'a Config, arg: &str) -> Option<CacheQuery<'a>> {
    if let Some(caps) = REGEX_PACKAGE_QUERY.captures(arg) {
        let mut query = CacheQuery::new(config);
        if let Some(name) = caps.name("package") {
            query.with_name(name.as_str());
        }
        if let Some(category) = caps.name("category") {
            query.with_category(category.as_str());
        }
        if let Some(repository) = caps.name("repository") {
            query.with_repository(repository.as_str());
        }
        Some(query)
    } else {
        None
    }
}

/// Looks for a unique package with the given name.
pub fn packages<'a>(config: &'a Config, names: &[String]) -> Result<Vec<Package<'a>>, Error> {
    let mut targets = Vec::new();

    for target in names {
        if let Some(query) = cache(config, &target) {
            let mut packages = query.perform()?;
            match packages.len() {
                0 => Err(QueryErrorKind::NoResult(target.to_string())),
                1 => {
                    targets.append(&mut packages);
                    Ok(())
                }
                _ => {
                    let vec = packages
                        .iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<String>>();
                    Err(QueryErrorKind::TooManyResults(
                        target.to_string(),
                        vec.len(),
                        vec,
                    ))
                }
            }
        } else {
            Err(QueryErrorKind::InvalidPackageName(target.to_string()))
        }?;
    }
    Ok(targets)
}

//! Functions to resolve package's name

use failure::Error;
use libnest::config::Config;
use libnest::package::{Package, PackageRequirement};
use libnest::query::CacheQuery;

use error::QueryErrorKind;

/// Looks for a unique package with the given requirement
pub fn packages<'a>(config: &'a Config, requirements: &[PackageRequirement]) -> Result<Vec<Package<'a>>, Error> {
    let mut targets = Vec::new();

    for target in requirements {
        let query = CacheQuery::from_requirement(config, target.clone());
        let mut packages = query.perform()?;
        match packages.len() {
            0 => Err(QueryErrorKind::NoResult(target.to_string()))?,
            1 => targets.append(&mut packages),
            _ => {
                let vec = packages
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>();
                Err(QueryErrorKind::TooManyResults(
                    target.to_string(),
                    vec.len(),
                    vec,
                ))?
            }
        }
    }
    Ok(targets)
}

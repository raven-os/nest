//! Provides function to resolve package name

use regex::Regex;

use libnest::query::CacheQuery;
use libnest::config::Config;

lazy_static! {
    static ref REGEX_PACKAGE_QUERY: Regex = Regex::new(r"^((?P<repository>[a-z]+)::)?((?P<category>[a-z\-]+)/)?(?P<package>([a-z_]+))$").unwrap();
}

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

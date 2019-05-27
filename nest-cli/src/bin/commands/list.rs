use clap::ArgMatches;
use failure::Error;

use libnest::cache::depgraph::{NodeKind, RequirementManagementMethod};
use libnest::config::Config;

pub fn list(config: &Config, matches: &ArgMatches) -> Result<(), Error> {
    let lock_file_ownership = config.acquire_lock_file_ownership(true)?;
    let depgraph = config.dependency_graph(&lock_file_ownership)?;

    let mut packages = Vec::new();

    if matches.is_present("with-deps") {
        packages = depgraph
            .packages()
            .iter()
            .map(|(name, _)| format!("{}", name))
            .collect();
    } else {
        for (_, req) in depgraph.requirements() {
            if let RequirementManagementMethod::Static = req.management_method() {
                let node = depgraph
                    .nodes()
                    .get(&req.fulfilling_node_id().unwrap())
                    .unwrap();

                if let NodeKind::Package { id } = node.kind() {
                    packages.push(format!(
                        "{}::{}/{}",
                        id.repository(),
                        id.category(),
                        id.name()
                    ));
                }
            }
        }
    }
    packages.sort();

    for p in packages {
        println!("{}", p);
    }
    Ok(())
}

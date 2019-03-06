use std::str::FromStr;

use clap::ArgMatches;
use failure::{format_err, Error};
use libnest::cache::available::AvailablePackagesCacheQueryStrategy;
use libnest::cache::depgraph::{GroupName, RequirementKind, RequirementManagementMethod};
use libnest::config::Config;
use libnest::package::{HardPackageRequirement, PackageRequirement};

pub fn requirement_add(
    config: &Config,
    target_group: &str,
    matches: &ArgMatches,
) -> Result<(), Error> {
    let group = GroupName::from_str(target_group)?;

    let mut scratch_graph = if config.paths().scratch_depgraph().exists() {
        config.scratch_dependency_graph()?
    } else {
        config.dependency_graph()?
    };

    let group_id = *scratch_graph
        .groups()
        .get(&group)
        .ok_or_else(|| format_err!("Unknown group"))?;
    let packages_cache = config.available_packages_cache();

    for target in &matches.values_of_lossy("PACKAGE").unwrap() {
        let requirement = PackageRequirement::parse(&target)?;

        let matched_packages = packages_cache
            .query(&requirement)
            .set_strategy(AvailablePackagesCacheQueryStrategy::BestMatch)
            .perform()?;
        if matched_packages.len() > 1 {
            for pkg in matched_packages {
                println!("{}", &pkg);
            }
            return Err(format_err!("unable to select a best match"));
        } else if matched_packages.is_empty() {
            return Err(format_err!(
                "no package found for requirement '{}'",
                &target
            ));
        }
        let package_req = HardPackageRequirement::from(
            matched_packages[0].full_name(),
            requirement.version_requirement().clone(),
        );

        println!("Adding requirement {} to group {}...", package_req, *group);
        scratch_graph.node_add_requirement(
            group_id,
            RequirementKind::Package { package_req },
            RequirementManagementMethod::Static,
        );
    }

    scratch_graph.solve(&config)?;

    scratch_graph.save_to_cache(config.paths().scratch_depgraph())?;

    Ok(())
}

pub fn requirement_remove(
    config: &Config,
    target_group: &str,
    matches: &ArgMatches,
) -> Result<(), Error> {
    let group = GroupName::from_str(target_group)?;

    let mut graph = if config.paths().scratch_depgraph().exists() {
        config.scratch_dependency_graph()?
    } else {
        config.dependency_graph()?
    };
    let original_graph = graph.clone();

    let group_id = *graph
        .groups()
        .get(&group)
        .ok_or_else(|| format_err!("Unknown group"))?;

    {
        let packages_cache = config.available_packages_cache();

        for target in &matches.values_of_lossy("PACKAGE").unwrap() {
            let requirement = PackageRequirement::parse(&target)?;

            let matches = packages_cache.query(&requirement).perform()?;

            let group_node = original_graph.nodes().get(&group_id).unwrap();

            let found = matches.iter().any(|pkg| {
                group_node.requirements().iter().any(|req_id| {
                    let req = graph.requirements().get(req_id).unwrap();
                    if let RequirementKind::Package { package_req } = req.kind() {
                        if package_req.full_name() == &pkg.full_name() {
                            println!(
                                "Removing requirement {} from group {}...",
                                package_req, *group
                            );
                            graph.remove_requirement(*req_id);
                            return true;
                        }
                    }
                    false
                })
            });

            if !found {
                return Err(format_err!(
                    "unable to find an installed package matching '{}'",
                    &target
                ));
            }
        }
    }

    graph.solve(&config)?;

    graph.save_to_cache(config.paths().scratch_depgraph())?;

    Ok(())
}

use std::str::FromStr;

use clap::ArgMatches;
use failure::{format_err, Error};
use libnest::cache::depgraph::{GroupName, RequirementKind, RequirementManagementMethod};
use libnest::config::Config;

pub fn group_add(config: &Config, parent_group: &str, matches: &ArgMatches) -> Result<(), Error> {
    let parent_group = GroupName::from_str(parent_group)?;

    let lock_file_ownership = config.acquire_lock_file_ownership(true)?;

    let mut graph = config.scratch_dependency_graph(&lock_file_ownership)?;

    let parent_group_id = *graph
        .node_names()
        .get(&parent_group.clone().into())
        .ok_or_else(|| format_err!("Unknown parent group {}", *parent_group))?;

    for group in matches.values_of_lossy("GROUP").unwrap() {
        let group_name = GroupName::from_str(group.as_str())?;
        println!(
            "Adding group {} with parent group {}...",
            *group_name, *parent_group
        );
        graph.add_group_node(group_name.clone())?;
        graph.node_add_requirement(
            parent_group_id,
            RequirementKind::Group { name: group_name },
            RequirementManagementMethod::Static,
        );
    }

    graph.solve(config)?;

    graph.save_to_cache(config.paths().scratch_depgraph(), &lock_file_ownership)?;

    println!("Successfully added all the specified groups.");

    Ok(())
}

pub fn group_remove(config: &Config, matches: &ArgMatches) -> Result<(), Error> {
    let lock_file_ownership = config.acquire_lock_file_ownership(true)?;

    let mut graph = config.scratch_dependency_graph(&lock_file_ownership)?;

    for group in matches.values_of_lossy("GROUP").unwrap() {
        let group_name = GroupName::from_str(group.as_str())?;
        println!("Removing group {}...", *group_name);
        graph.node_remove_requirement(graph.root_id(), RequirementKind::Group { name: group_name });
    }

    graph.solve(config)?;

    graph.save_to_cache(config.paths().scratch_depgraph(), &lock_file_ownership)?;

    println!("Successfully removed all the specified groups.");

    Ok(())
}

pub fn group_list(config: &Config) -> Result<(), Error> {
    let lock_file_ownership = config.acquire_lock_file_ownership(true)?;

    let graph = config.scratch_dependency_graph(&lock_file_ownership)?;

    for group_name in graph.groups() {
        println!("{}", group_name.as_str());
    }

    Ok(())
}

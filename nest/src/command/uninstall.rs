//! Functions to execute the `uninstall` operation.

use clap::ArgMatches;
use failure::Error;

use libnest::cache::depgraph::{DependencyGraphDiff, RequirementKind};
use libnest::config::Config;
use libnest::package::PackageRequirement;
use libnest::transaction::Orchestrator;

use command;

/// Uninstalls all the given packages.
///
/// This will go through all targets, remove them of the dependency graph and perform the operations that are
/// needed in order to uninstall the packages.
pub fn uninstall(config: &Config, matches: &ArgMatches) -> Result<(), Error> {
    // Remove arguments as requirements of the root node.
    let mut graph = config.depgraph()?;
    let original_graph = graph.clone();
    {
        let root = graph.root_id();
        for target in &matches.values_of_lossy("PACKAGE").unwrap() {
            let requirement = PackageRequirement::parse(target)?;
            graph.remove_requirement(
                root,
                &RequirementKind::Package {
                    package_req: requirement,
                },
            )?;
        }
    }

    let diff = DependencyGraphDiff::new();
    let transactions = diff.perform(&original_graph, &graph.clone())?;

    if transactions.is_empty() {
        println!("All given requirements are already satifisfied. There is nothing to do.");
    } else {
        // Calculate the transactions that we have to do, setup the orchestrator
        let orchestrator = Orchestrator::from(transactions);

        // Perform and monitor the transactions.
        command::orchestrate(config, orchestrator)?;

        // Save the new dependency graph
        graph.save(config)?;
    }
    Ok(())
}

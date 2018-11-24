//! Functions to execute the `install` operation.

use clap::ArgMatches;
use failure::Error;

use libnest::cache::depgraph::DependencyGraphDiff;
use libnest::config::Config;
use libnest::package::PackageRequirement;
use libnest::transaction::Orchestrator;

use crate::command;

/// Installs all the given packages.
///
/// This will go through all targets, add them to the dependency graph and perform all the operations
/// needed in order to install the packages.
pub fn install(config: &Config, matches: &ArgMatches) -> Result<(), Error> {
    // Add arguments as requirements of the root node.
    let mut graph = config.depgraph()?;
    let original_graph = graph.clone();
    {
        let root = graph.root_id();
        for target in &matches.values_of_lossy("PACKAGE").unwrap() {
            graph.add_package(config, root, PackageRequirement::parse(target)?)?;
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

//! Functions to execute the `install` operation.

use clap::ArgMatches;
use failure::Error;

use libnest::config::Config;
use libnest::package::PackageRequirement;
use libnest::transaction::Orchestrator;

use command;

/// Installs all the given packages.
///
/// This will go through all targets, check that they exist, resolve the dependency graph,
/// download the packages, ensure the installation will not break anything nor delete any file
/// and then install the package.
pub fn install(config: &Config, matches: &ArgMatches) -> Result<(), Error> {
    // Add arguments as requirements of the root node.

    let mut graph = config.depgraph()?;
    let original_graph = graph.clone();
    {
        let root = graph.root_id();
        for target in &matches.values_of_lossy("PACKAGE").unwrap() {
            let requirement = PackageRequirement::parse(target)?;
            graph.add_package(config, root, &requirement)?;
        }
    }

    let transactions = original_graph - graph.clone();

    if transactions.len() == 0 {
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

//! Functions to execute the `update` operation.

use failure::Error;

use libnest::cache::depgraph::DependencyGraphDiff;
use libnest::config::Config;
use libnest::transaction::Orchestrator;

use command;

/// Updates all packages.
pub fn upgrade(config: &Config) -> Result<(), Error> {
    let mut graph = config.depgraph()?;
    let original_graph = graph.clone();

    let root = graph.root_id();
    graph.update_node(root)?;

    let diff = DependencyGraphDiff::new();
    let transactions = diff.perform(&original_graph, &graph.clone())?;

    if transactions.is_empty() {
        println!("All packages up to date: there is nothing to do.");
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

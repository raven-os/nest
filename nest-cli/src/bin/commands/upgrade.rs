use clap::ArgMatches;
use failure::Error;
use libnest::cache::depgraph::DependencyGraphDiff;
use libnest::config::Config;

use super::{
    ask_confirmation, download_required_packages, print_transactions, process_transactions,
};

pub fn upgrade(config: &Config, _: &ArgMatches) -> Result<(), Error> {
    let lock_file_ownership = config.acquire_lock_file_ownership(true)?;
    let mut graph = config.dependency_graph(&lock_file_ownership)?;
    let original_graph = graph.clone();

    graph.update(config)?;

    let transactions = DependencyGraphDiff::new().perform(&original_graph, &graph);

    if transactions.is_empty() {
        println!("All the given requirements are already satisfied, quitting.");
        return Ok(());
    }

    print_transactions(&transactions);

    if !ask_confirmation(
        format!(
            "Would you like to apply th{} transaction{}?",
            if transactions.len() <= 1 { "is" } else { "ese" },
            if transactions.len() <= 1 { "" } else { "s" },
        )
        .as_str(),
        true,
    )? {
        println!(
            "Transaction{} cancelled.",
            if transactions.len() <= 1 { "" } else { "s" }
        );
        return Ok(());
    }

    download_required_packages(config, &transactions, &lock_file_ownership)?;

    process_transactions(config, &transactions, &lock_file_ownership)?;

    graph.save_to_cache(config.paths().depgraph(), &lock_file_ownership)?;

    Ok(())
}

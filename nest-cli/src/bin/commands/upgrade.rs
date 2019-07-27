use clap::ArgMatches;
use failure::Error;
use libnest::cache::depgraph::DependencyGraphDiff;
use libnest::config::Config;
use libnest::transaction::Transaction;

use super::operations::download::download_packages;
use super::{ask_confirmation, print_transactions, process_transactions};

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

    println!("Downloading packages...");
    let downloaded = config.downloaded_packages_cache(&lock_file_ownership);
    download_packages(
        config,
        transactions.iter().filter_map(|trans| match trans {
            Transaction::Install(install) if !downloaded.has_package(install.target()) => {
                Some(install.associated_download())
            }
            Transaction::Upgrade(upgrade) if !downloaded.has_package(upgrade.new_target()) => {
                Some(upgrade.associated_download())
            }
            _ => None,
        }),
    )?;

    process_transactions(config, &transactions, &lock_file_ownership)?;

    graph.save_to_cache(config.paths().depgraph(), &lock_file_ownership)?;

    Ok(())
}

use failure::{format_err, Error, ResultExt};
use libnest::cache::depgraph::DependencyGraphDiff;
use libnest::config::Config;
use libnest::transaction::Transaction;

use super::operations::download::download_packages;
use super::operations::install::install_package;
use super::operations::uninstall::uninstall_package;
use super::{ask_confirmation, print_transactions};
use crate::commands::operations::upgrade::upgrade_package;

pub fn merge(config: &Config) -> Result<(), Error> {
    let lock_file_ownership = config.acquire_lock_file_ownership(true)?;

    let graph = config
        .scratch_dependency_graph(&lock_file_ownership)
        .with_context(|_| format_err!("no scratch dependency graph found"))?;
    let original_graph = config.dependency_graph(&lock_file_ownership)?;

    let transactions = DependencyGraphDiff::new().perform(&original_graph, &graph);

    if transactions.is_empty() {
        println!("No transactions are required, quitting.");
        graph.save_to_cache(config.paths().depgraph(), &lock_file_ownership)?;
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
    download_packages(
        config,
        transactions.iter().filter_map(|trans| match trans {
            Transaction::Install(install) => Some(install.associated_download()),
            Transaction::Upgrade(upgrade) => Some(upgrade.associated_download()),
            _ => None,
        }),
    )?;

    for transaction in transactions.iter() {
        match transaction {
            Transaction::Install(install) => {
                install_package(config, install, &lock_file_ownership)?
            }
            Transaction::Remove(remove) => uninstall_package(config, remove, &lock_file_ownership)?,
            Transaction::Upgrade(upgrade) => {
                upgrade_package(config, upgrade, &lock_file_ownership)?
            }
            _ => unimplemented!(),
        };
    }

    graph.save_to_cache(config.paths().depgraph(), &lock_file_ownership)?;

    Ok(())
}

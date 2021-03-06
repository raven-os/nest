use clap::ArgMatches;
use failure::{format_err, Error};
use libnest::cache::depgraph::{DependencyGraphDiff, RequirementKind};
use libnest::config::Config;
use libnest::package::SoftPackageRequirement;

use super::{ask_confirmation, print_transactions, process_transactions};

pub fn uninstall(config: &Config, matches: &ArgMatches) -> Result<(), Error> {
    let lock_file_ownership = config.acquire_lock_file_ownership(true)?;

    let mut graph = config.dependency_graph(&lock_file_ownership)?;
    let original_graph = graph.clone();

    {
        let packages_cache = config.available_packages_cache(&lock_file_ownership);

        for target in &matches.values_of_lossy("PACKAGE").unwrap() {
            let requirement = SoftPackageRequirement::parse(&target)?;

            let matches = packages_cache.query(&requirement).perform()?;

            let root_node = graph.nodes().get(&graph.root_id()).unwrap().clone();

            let found = matches.iter().any(|pkg| {
                root_node.requirements().iter().any(|req_id| {
                    let req = graph.requirements().get(req_id).unwrap();
                    if let RequirementKind::Package { package_req } = req.kind() {
                        let full_name = pkg.full_name();
                        if package_req.matches_full_name_precisely(&full_name) {
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

    process_transactions(config, &transactions, &lock_file_ownership)?;

    graph.save_to_cache(config.paths().depgraph(), &lock_file_ownership)?;

    Ok(())
}

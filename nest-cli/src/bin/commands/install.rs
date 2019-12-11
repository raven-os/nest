use clap::ArgMatches;
use failure::{format_err, Error};
use libnest::cache::available::AvailablePackagesCacheQueryStrategy;
use libnest::cache::depgraph::{DependencyGraphDiff, RequirementKind, RequirementManagementMethod};
use libnest::config::Config;
use libnest::package::{HardPackageRequirement, SoftPackageRequirement};

use super::{
    ask_confirmation, download_required_packages, print_transactions, process_transactions,
};

pub fn install(config: &Config, matches: &ArgMatches) -> Result<(), Error> {
    let lock_file_ownership = config.acquire_lock_file_ownership(true)?;

    let mut graph = config.dependency_graph(&lock_file_ownership)?;
    let original_graph = graph.clone();

    {
        let packages_cache = config.available_packages_cache(&lock_file_ownership);

        for target in &matches.values_of_lossy("PACKAGE").unwrap() {
            let requirement = SoftPackageRequirement::parse(&target)?;

            let matched_packages = packages_cache
                .query(&requirement)
                .set_strategy(AvailablePackagesCacheQueryStrategy::BestMatch)
                .perform_and_sort_by_preference(config)?;
            if matched_packages.is_empty() {
                return Err(format_err!(
                    "no package found for requirement '{}'",
                    &target
                ));
            }
            let matched_package = &matched_packages[0];

            let package_req = HardPackageRequirement::from(
                matched_package.full_name(),
                requirement.version_requirement().clone(),
            );
            graph.node_add_requirement(
                graph.root_id(),
                RequirementKind::Package {
                    package_req: package_req.into(),
                },
                RequirementManagementMethod::Static,
            );
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

    download_required_packages(config, &transactions, &lock_file_ownership)?;

    process_transactions(config, &transactions, &lock_file_ownership)?;

    graph.save_to_cache(config.paths().depgraph(), &lock_file_ownership)?;

    Ok(())
}

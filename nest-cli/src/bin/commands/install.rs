use clap::ArgMatches;
use failure::{format_err, Error};
use libnest::cache::available::AvailablePackagesCacheQueryStrategy;
use libnest::cache::depgraph::{DependencyGraphDiff, RequirementKind, RequirementManagementMethod};
use libnest::config::Config;
use libnest::package::{HardPackageRequirement, PackageRequirement};
use libnest::transaction::Transaction;

use super::operations::install::install_package;
use super::{ask_confirmation, print_transactions};

pub fn install(config: &Config, matches: &ArgMatches) -> Result<(), Error> {
    let mut graph = config.dependency_graph()?;
    let original_graph = graph.clone();

    {
        let packages_cache = config.available_packages_cache();

        for target in &matches.values_of_lossy("PACKAGE").unwrap() {
            let requirement = PackageRequirement::parse(&target)?;

            let matched_packages = packages_cache
                .query(&requirement)
                .set_strategy(AvailablePackagesCacheQueryStrategy::BestMatch)
                .perform()?;
            if matched_packages.len() > 1 {
                for pkg in matched_packages {
                    println!("{}", &pkg);
                }
                return Err(format_err!("unable to select a best match"));
            } else if matched_packages.is_empty() {
                return Err(format_err!(
                    "no package found for requirement '{}'",
                    &target
                ));
            }
            let package_req = HardPackageRequirement::from(
                matched_packages[0].full_name(),
                requirement.version_requirement().clone(),
            );
            graph.node_add_requirement(
                graph.root_id(),
                RequirementKind::Package { package_req },
                RequirementManagementMethod::Static,
            );
        }
    }

    graph.solve(&config)?;

    let mut transactions = DependencyGraphDiff::new().perform(&original_graph, &graph);

    if transactions.is_empty() {
        println!("All the given requirements are already satisfied, quitting.");
        return Ok(());
    }

    print_transactions(&transactions);

    if !ask_confirmation(
        format!(
            "Would you like to apply th{} transaction{} ?",
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

    {
        let lock_file_ownership = config.acquire_lock_file_ownership(true)?;

        for mut transaction in &mut transactions.iter_mut() {
            match &mut transaction {
                Transaction::Install(install) => {
                    install_package(config, install, &lock_file_ownership)?
                }
                _ => unimplemented!(),
            };
        }

        graph.save_to_cache(config.paths().depgraph(), &lock_file_ownership)?;
    }

    Ok(())
}

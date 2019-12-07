use clap::ArgMatches;
use failure::{format_err, Error};
use libnest::cache::depgraph::NodeKind;
use libnest::config::Config;
use libnest::package::SoftPackageRequirement;
use libnest::transaction::{InstallTransaction, RemoveTransaction};

use super::operations::download::download_packages;
use super::operations::install::install_package;
use super::operations::uninstall::uninstall_package;

pub fn reinstall(config: &Config, matches: &ArgMatches) -> Result<(), Error> {
    let lock_file_ownership = config.acquire_lock_file_ownership(true)?;

    let graph = config.dependency_graph(&lock_file_ownership)?;

    let packages = matches
        .values_of_lossy("PACKAGE")
        .unwrap()
        .iter()
        .map(|s| {
            SoftPackageRequirement::parse(s).and_then(|package_req| {
                let matching_installed_packages = graph
                    .nodes()
                    .values()
                    .map(|node| node.kind())
                    .filter_map(NodeKind::package)
                    .filter(|pkg_id| package_req.matches_precisely(&pkg_id))
                    .collect::<Vec<_>>();

                match matching_installed_packages.len() {
                    1 => Ok(matching_installed_packages[0].clone()),
                    0 => Err(format_err!("no package matches the {} requirement", &s)),
                    _ => Err(format_err!(
                        "multiple installed packages match the {} requirement, please disambiguate",
                        &s
                    )),
                }
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let (installs, removals): (Vec<_>, Vec<_>) = packages
        .into_iter()
        .map(|pkg_id| {
            (
                InstallTransaction::from(pkg_id.clone()),
                RemoveTransaction::from(pkg_id),
            )
        })
        .unzip();

    println!("Downloading packages...");
    download_packages(
        config,
        installs.iter().map(InstallTransaction::associated_download),
    )?;

    for (install, removal) in installs.into_iter().zip(removals.into_iter()) {
        uninstall_package(config, &removal, &lock_file_ownership)?;
        install_package(config, &install, &lock_file_ownership)?;
    }

    Ok(())
}

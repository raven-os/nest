mod group;
mod install;
mod list;
mod merge;
pub mod operations;
mod pull;
mod reinstall;
mod requirement;
mod uninstall;
mod upgrade;

pub use self::group::{group_add, group_list, group_remove};
pub use self::install::install;
pub use self::list::list;
pub use self::merge::merge;
use self::operations::download::{download_hashes, download_packages};
use self::operations::install::install_package;
use self::operations::uninstall::uninstall_package;
use self::operations::upgrade::upgrade_package;
pub use self::pull::pull;
pub use self::reinstall::reinstall;
pub use self::requirement::{requirement_add, requirement_remove};
pub use self::uninstall::uninstall;
pub use self::upgrade::upgrade;

use colored::*;
use failure::{Error, ResultExt};
use std::io::{self, Write};

use libnest::config::Config;
use libnest::lock_file::LockFileOwnership;
use libnest::transaction::Transaction;

pub fn print_transactions(transactions: &[Transaction]) {
    println!(
        "{}",
        format!(
            "{} pending transaction{}:",
            transactions.len(),
            if transactions.len() <= 1 { "" } else { "s" }
        )
        .bold()
    );
    println!();
    for transaction in transactions {
        println!(
            "{}",
            match transaction {
                Transaction::Pull(p) => {
                    format!("{:>10.10} {}", "pull".cyan(), p.target_repository().name()).bold()
                }
                Transaction::Install(i) => {
                    format!("{:>10.10} {}", "install".green(), i.target()).bold()
                }
                Transaction::Remove(r) =>
                    format!("{:>10.10} {}", "remove".red(), r.target()).bold(),
                Transaction::Upgrade(u) => {
                    format!("{:>10.10} {}", "upgrade".yellow(), u.new_target()).bold()
                }
            }
        );
    }
}

pub fn ask_confirmation(question: &str, default: bool) -> Result<bool, Error> {
    let hint = if default {
        format!("{}/{}", "Yes".green().bold(), "no".red().bold())
    } else {
        format!("{}/{}", "yes".green().bold(), "No".red().bold())
    };

    print!("\n{} [{}] ", question.bold(), hint);
    loop {
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input).context("stdin")?;

        match input.trim().to_lowercase().as_ref() {
            "" => return Ok(default),
            "y" | "yes" => return Ok(true),
            "n" | "no" => return Ok(false),
            _ => print!("Please type \"yes\" or \"no\". [{}] ", hint),
        }
    }
}

pub fn process_transactions(
    config: &Config,
    transactions: &[Transaction],
    lock_file_ownership: &LockFileOwnership,
) -> Result<(), Error> {
    for transaction in transactions.iter() {
        match transaction {
            Transaction::Install(install) => {
                install_package(config, install, &lock_file_ownership)?
            }
            Transaction::Upgrade(upgrade) => {
                upgrade_package(config, upgrade, &lock_file_ownership)?
            }
            Transaction::Remove(remove) => uninstall_package(config, remove, &lock_file_ownership)?,
            _ => unimplemented!(),
        };
    }
    Ok(())
}

pub fn download_required_packages(
    config: &Config,
    transactions: &[Transaction],
    lock_file_ownership: &LockFileOwnership,
) -> Result<(), Error> {
    println!("Checking for packages to download...");

    let downloaded_cache = config.downloaded_packages_cache(lock_file_ownership);

    let downloads = transactions.iter().filter_map(|trans| match trans {
        Transaction::Install(install) => Some(install.associated_download()),
        Transaction::Upgrade(upgrade) => Some(upgrade.associated_download()),
        _ => None,
    });

    // List all the packages that are not present in the download cache, and thus must be downloaded
    let never_downloaded = downloads
        .clone()
        .filter(|download| !downloaded_cache.has_package(download.target()));

    // List the packages that are already in the cache
    let already_downloaded =
        downloads.filter(|download| downloaded_cache.has_package(download.target()));

    // Retrieve (download, server-issued hash) pairs for packages that are in the cache
    let downloads_with_hashes = download_hashes(config, already_downloaded)?;

    // Check correspondence of each served-issued hash with the local hash
    let downloads_with_validities = downloads_with_hashes
        .map(|(download, hash)| {
            downloaded_cache
                .has_package_matching_hash(download.target(), &hash)
                .map(|valid| (download, valid))
        })
        .collect::<Result<Vec<_>, _>>()?;

    // Keep only the packages that are in the cache but whose hashes do not match the server's
    let downloaded_with_stale_hashes =
        downloads_with_validities
            .into_iter()
            .filter_map(
                |(download, valid)| {
                    if !valid {
                        Some(download)
                    } else {
                        None
                    }
                },
            );

    // Get a full list of the packages that need to be downloaded
    let to_download = never_downloaded.chain(downloaded_with_stale_hashes);

    let mut downloads_to_print = to_download.clone().peekable();
    if downloads_to_print.peek().is_some() {
        println!();
        for download in downloads_to_print {
            println!(
                "{}",
                format!("{:>10.10} {}", "download".cyan(), download.target()).bold()
            );
        }
        println!();

        println!("Downloading packages...");
        download_packages(config, to_download)
    } else {
        println!("No packages need to be downloaded.");
        Ok(())
    }
}

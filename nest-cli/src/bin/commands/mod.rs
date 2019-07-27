mod group;
mod install;
mod list;
mod merge;
pub mod operations;
mod pull;
mod requirement;
mod uninstall;
mod upgrade;

pub use self::group::{group_add, group_list, group_remove};
pub use self::install::install;
pub use self::list::list;
pub use self::merge::merge;
use self::operations::install::install_package;
use self::operations::uninstall::uninstall_package;
use self::operations::upgrade::upgrade_package;
pub use self::pull::pull;
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
                    format!("{:>8.8} {}", "pull".cyan(), p.target_repository().name()).bold()
                }
                Transaction::Install(i) => {
                    format!("{:>8.8} {}", "install".green(), i.target()).bold()
                }
                Transaction::Remove(r) => format!("{:>8.8} {}", "remove".red(), r.target()).bold(),
                Transaction::Upgrade(u) => {
                    format!("{:>8.8} {}", "upgrade".yellow(), u.new_target()).bold()
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

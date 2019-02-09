pub mod operations;
mod pull;

pub use self::pull::pull;

use colored::*;
use failure::{Error, ResultExt};
use std::io::{self, Write};

use libnest::config::Config;
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
                    format!("{} {}", "pull".cyan(), p.target_repository().name()).bold()
                }
                Transaction::Install(_) => "install".green().bold(),
                Transaction::Remove(_) => "remove".red().bold(),
                Transaction::Upgrade(_) => "upgrade".yellow().bold(),
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

pub fn install(_config: &Config) -> Result<(), Error> {
    Ok(())
}

pub fn uninstall(_config: &Config) -> Result<(), Error> {
    Ok(())
}

pub fn upgrade(_config: &Config) -> Result<(), Error> {
    Ok(())
}

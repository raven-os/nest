use failure::{format_err, Error, ResultExt};
use indicatif::{ProgressBar, ProgressStyle};
use libnest::config::Config;
use libnest::transaction::{PullTransaction, Transaction};

use super::download::Download;
use super::{ask_confirmation, print_transactions};

pub fn pull(config: &Config) -> Result<(), Error> {
    let transactions: Vec<_> = config
        .repositories()
        .into_iter()
        .map(|repository| Transaction::Pull(PullTransaction::from(repository)))
        .collect();

    if transactions.is_empty() {
        println!("No repository to pull, quitting.");
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

    let progress_bar = ProgressBar::new(transactions.len() as u64);
    progress_bar.set_style(ProgressStyle::default_bar().template("[{pos:>3}/{len:3}] {bar:80}"));

    let mut transactions = transactions;
    let download = Download::from("pull");

    for pull in transactions.iter_mut() {
        if let Transaction::Pull(pull) = pull {
            let repo = *pull.target_repository();

            progress_bar.println(format!("Pulling {}...", repo.name()).as_str());

            download
                .perform_with_mirrors(&mut pull.writer(), repo.config().mirrors())
                .context(format_err!("unable to pull repository '{}'", repo.name()))?;
            pull.save_to_cache(config)?;

            progress_bar.inc(1);
        }
    }
    progress_bar.finish_and_clear();
    println!(
        "Successfully pulled {} repositor{}",
        transactions.len(),
        if transactions.len() <= 1 { "y" } else { "ies" }
    );
    Ok(())
}

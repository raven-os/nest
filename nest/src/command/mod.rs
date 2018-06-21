//! Types and functions to execute all of nest's operations.

pub mod install;
pub mod pull;

use std::io::{self, Write};

use failure::{Error, ResultExt};
use libnest::config::Config;
use libnest::transaction::{Notification, Notifier, Orchestrator, TransactionKind};

use progressbar::ProgressBar;

pub fn yesno(question: &str, default: bool) -> Result<bool, Error> {
    let yesno = if default {
        format!("{}/{}", green!("Yes"), red!("no"))
    } else {
        format!("{}/{}", green!("yes"), red!("No"))
    };

    print!("\n{} [{}] ", bold!(question), yesno);
    loop {
        io::stdout().flush()?;
        let mut s = String::new();

        io::stdin().read_line(&mut s).context("stdin")?;

        match s.trim().to_lowercase().as_ref() {
            "" => return Ok(default),
            "y" | "yes" | "true" => return Ok(true),
            "n" | "no" | "false" => return Ok(false),
            s @ _ => print!("Sory, \"{}\" isn't a valid answer. [{}] ", s, yesno),
        }
    }
}

pub fn orchestrate(config: &Config, mut orchestrator: Orchestrator) -> Result<(), Error> {
    for transaction in orchestrator.transactions() {
        println!(
            " {} {}",
            match transaction.kind() {
                TransactionKind::Pull => cyan!("{:>8.8}", "pull"),
                TransactionKind::Install => green!("{:<8.8}", "install"),
            },
            transaction.target(),
        );
    }

    if yesno("Would you like to apply these transactions?", true)? {
        let mut pbs = orchestrator
            .transactions()
            .iter()
            .map(|transaction| {
                ProgressBar::new(transaction.kind(), transaction.target().to_string())
            })
            .collect::<Vec<_>>();

        let mut notifier = Notifier::new(
            // Notification
            |transaction, notification| {
                let pb = pbs
                    .get_mut(transaction.idx())
                    .expect("Transaction has an invalid id");
                match notification {
                    Notification::NewStep(step, retry) => {
                        if retry {
                            pb.retry();
                        } else {
                            pb.next_step(step);
                        }
                    }
                    Notification::Progress(current, max) => pb.update(current, max),
                    Notification::FinishTransaction(res) => pb.finish(res),
                    Notification::Warning(error) => {
                        eprintln!("{}: {}", purple!("warning"), format_error_causes!(error))
                    }
                }
            },
        );
        orchestrator.perform(config, &mut notifier)?;
    }
    Ok(())
}

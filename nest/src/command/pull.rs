//! Functions to execute the `pull` operation.

use failure::Error;
use libnest::config::Config;
use libnest::transaction::{Orchestrator, Pull, Transaction};

use command;

/// Pulls all repositories.
///
/// This creates an orchestrator that will pull all repositories and setups a bunch of callbacks to alert
/// the user of what's happening.
pub fn pull(config: &Config) -> Result<(), Error> {
    // Create all the pull transactions
    let pulls: Vec<_> = config
        .repositories()
        .into_iter()
        .map(|repository| Box::new(Pull::from(repository)) as Box<Transaction>)
        .collect();

    // Setup the orchestrator
    let orchestrator = Orchestrator::from(pulls);

    // Perform and monitor the transactions.
    command::orchestrate(config, orchestrator)
}

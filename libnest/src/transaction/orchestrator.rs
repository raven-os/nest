use failure::{Error, ResultExt};

use crate::config::Config;
use crate::transaction::{Notification, Notifier, Transaction};

/// The orchestrator takes a collection of transactions and performs them in a more efficient
/// way (possibly using multiple threads).
#[derive(Debug)]
pub struct Orchestrator<'a> {
    transactions: Vec<Box<Transaction + 'a>>,
}

impl<'a> Orchestrator<'a> {
    /// Creates an [`Orchestrator`] from a [`Vec`]<[`Box`]<[`Transaction>`]>>.
    #[inline]
    pub fn from(transactions: Vec<Box<Transaction + 'a>>) -> Orchestrator<'a> {
        Orchestrator { transactions }
    }

    /// Returns a reference over the [`Transaction`] contain within this [`Orchestrator`].
    #[inline]
    pub fn transactions(&self) -> &Vec<Box<Transaction + 'a>> {
        &self.transactions
    }

    /// Performs all the transactions stored in this orchestrator.
    ///
    /// It may use multiple threads to run transactions concurently.
    #[inline]
    pub fn perform(&mut self, config: &Config, notifier: &mut Notifier) -> Result<(), Error> {
        for (i, transaction) in self.transactions.iter_mut().enumerate() {
            transaction.assign_idx(i);
            let res: Result<_, Error> = transaction
                .perform(config, notifier)
                .with_context(|_| transaction.target().to_string())
                .map_err(From::from);
            notifier.notify(transaction.as_ref(), Notification::FinishTransaction(&res));
            res?;
        }
        Ok(())
    }
}

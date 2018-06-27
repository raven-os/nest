//! Transactions and ways to notify the user about what happens when they are performed.
//!
//! This module contains a definition of a [`Transaction] in the form of a trait that all transactions
//! implement ([`Pull`], [`Install`], [`Remove`], [`Update`] etc.)
//!
//! It also contains a way to orchestrate multiple transactions so they can be performed in the most efficient way, and a notifier
//! to show the evolution of each one of them.

mod install;
mod notifier;
mod orchestrator;
mod pull;
mod remove;
mod transfer;
mod upgrade;

pub use self::install::Install;
pub use self::notifier::{Notification, Notifier};
pub use self::orchestrator::Orchestrator;
pub use self::pull::Pull;
pub use self::remove::Remove;
pub use self::transfer::Transfer;
pub use self::upgrade::Upgrade;

use std::fmt::Debug;
use std::fmt::{self, Display, Formatter};

use failure::Error;

use config::Config;

/// The kind of a [`Transaction`].
///
/// Any transaction can be identifid with this [`TransactionKind`].
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum TransactionKind {
    /// The transaction is a Pull
    Pull,
    /// The transaction is an Install
    Install,
    /// The transaction is a Remove
    Remove,
    /// The transaction is an Upgrade
    Upgrade,
    /// The transaction is a Downgrade
    Downgrade,
    /// The transaction is a Reinstall
    Reinstall,
}

/// The step a transaction may be in.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum TransactionStep {
    /// The transaction is waiting for something
    Waiting,
    /// The transaction is preparing itself
    Prepare,
    /// The transaction is downloading a file
    Download,
    /// The transaction is checking the filesystem's content or integrity
    Check,
    /// The transaction is extracting some files on the filesystem from an archive
    Extract,
    /// The transaction is removing some files of the filesystem
    Remove,
}

impl Display for TransactionStep {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            TransactionStep::Waiting => write!(f, "waiting"),
            TransactionStep::Prepare => write!(f, "prepare"),
            TransactionStep::Download => write!(f, "download"),
            TransactionStep::Check => write!(f, "check"),
            TransactionStep::Extract => write!(f, "extract"),
            TransactionStep::Remove => write!(f, "remove"),
        }
    }
}

/// A transaction is an interaction with the target's file system (Installation, Removal, Update etc.)
///
/// Transactions all implement this trait. It provied uniform way to perform and monitor
/// transactions, what ever they may do.
pub trait Transaction: Debug {
    /// Returns the number of this transaction out of the total number of transactions being performed at the same time.
    fn idx(&self) -> usize;
    /// Returns the total number of transactions being performed at the same time.
    fn assign_idx(&mut self, usize);

    /// Returns the kind of this transaction. This must be a constant for a given transaction.
    fn kind(&self) -> TransactionKind;

    /// Returns a stringified version of the target of this transaction.
    fn target(&self) -> &str;

    /// Performs the transaction.
    ///
    /// The given notifier is used to notify and monitor the progress of the transaction.
    /// Be aware, this may (and probably will) be a blocking call.
    ///
    /// In addition, multiple transactions may be performed concurently, it's the responsability
    /// of the transaction to protect itself from race conditions or any kind of concurrency issues.
    fn perform(&mut self, &Config, &mut Notifier) -> Result<(), Error>;
}

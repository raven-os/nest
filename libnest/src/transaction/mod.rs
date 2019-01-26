//! Provides types and functions to perform transactions.
//!
//! Within Nest, a transaction is any operation that could affect the host filesystem.
//! At the moment, we distinguish the following types of transactions:
//! - Pull
//! - Installation
//! - Removal
//!

mod install;
mod pull;
mod remove;

pub use self::install::InstallTransaction;
pub use self::pull::PullTransaction;
pub use self::remove::RemoveTransaction;

/// The kind of a transaction
///
/// Every transaction belongs to a kind
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Transaction<'a, 'b> {
    /// The transaction is a "pull" transaction
    Pull(PullTransaction<'a, 'b>),

    /// The transaction is an "install" transaction
    Install(InstallTransaction),

    /// The transaction is a "remove" transaction
    Remove(RemoveTransaction),
}

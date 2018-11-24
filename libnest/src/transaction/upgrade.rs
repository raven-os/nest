use failure::Error;

use crate::config::Config;
use crate::package::PackageId;
use crate::transaction::{Install, Notifier, Remove, Transaction, TransactionKind};

/// An `upgrade` transaction: it performs the upgrade of the target on the system.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Upgrade {
    old: PackageId,
    new: PackageId,
    target_name: String,
    idx: usize,
}

impl Upgrade {
    /// Creates an [`Upgrade`] from a target [`PackageId`].
    #[inline]
    pub fn from(old: PackageId, new: PackageId) -> Upgrade {
        let target_name = new.to_string();
        Upgrade {
            old,
            new,
            target_name,
            idx: 0,
        }
    }
}

impl Transaction for Upgrade {
    #[inline]
    fn idx(&self) -> usize {
        self.idx
    }

    #[inline]
    fn assign_idx(&mut self, idx: usize) {
        self.idx = idx;
    }

    #[inline]
    fn kind(&self) -> TransactionKind {
        if self.new.version() < self.old.version() {
            TransactionKind::Downgrade
        } else if self.new.version() > self.old.version() {
            TransactionKind::Upgrade
        } else {
            TransactionKind::Reinstall
        }
    }

    #[inline]
    fn target(&self) -> &str {
        &self.target_name
    }

    fn perform(&mut self, config: &Config, notifier: &mut Notifier) -> Result<(), Error> {
        // Quick & dirty, will be enhanced soon
        Remove::from(self.old.clone()).perform(config, notifier)?;
        Install::from(self.new.clone()).perform(config, notifier)?;
        Ok(())
    }
}

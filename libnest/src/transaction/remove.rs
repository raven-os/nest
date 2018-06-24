use std::fs::{self, File};
use std::io::{Seek, SeekFrom, Write};

use failure::{Error, ResultExt};
use flate2::read::GzDecoder;
use tar::Archive;

use config::Config;
use error::InstallError;
use package::PackageId;
use transaction::{Notification, Notifier, Transaction, TransactionKind, TransactionStep};

/// A `remove` transaction: it performs the removal of the target on the system.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Remove {
    target: PackageId,
    target_name: String,
    idx: usize,
}

impl Remove {
    /// Creates a [`Remove`] from a target [`PackageId`].
    #[inline]
    pub fn from(target: PackageId) -> Remove {
        let target_name = target.to_string();
        Remove {
            target,
            target_name,
            idx: 0,
        }
    }
}

impl Transaction for Remove {
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
        TransactionKind::Remove
    }

    #[inline]
    fn target(&self) -> &str {
        &self.target_name
    }

    fn perform(&mut self, config: &Config, notifier: &mut Notifier) -> Result<(), Error> {
        Ok(())
    }
}

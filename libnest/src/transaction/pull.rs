use std::io::Cursor;
use std::str;

use failure::{Error, ResultExt};
use json;

use config::Config;
use error::PullError;
use package::{Manifest, Package};
use repository::Repository;
use transaction::{Notification, Notifier, Transaction, TransactionKind, TransactionStep};

/// An `pull` transaction: it pulls the target repository and updates the local cache of available packages.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Pull<'a, 'b> {
    repository: Repository<'a, 'b>,
    idx: usize,
}

impl<'a, 'b> Pull<'a, 'b> {
    /// Creates a [`Pull`] transaction from a target [`Repository`].
    #[inline]
    pub fn from(repository: Repository<'a, 'b>) -> Pull<'a, 'b> {
        Pull { repository, idx: 0 }
    }
}

impl<'a, 'b> Transaction for Pull<'a, 'b> {
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
        TransactionKind::Pull
    }

    #[inline]
    fn target(&self) -> &str {
        self.repository.name()
    }

    fn perform(&mut self, config: &Config, notifier: &mut Notifier) -> Result<(), Error> {
        let mut data: Vec<u8> = Vec::new();
        let mut transfer = self.repository.transfer(config).target("pull".to_string());
        transfer.perform(&mut Cursor::new(&mut data), self, notifier)?;

        notifier.notify(self, Notification::NewStep(TransactionStep::Extract, false));

        // Unserialize received datas
        let res: Result<Vec<Manifest>, Error> = do catch {
            let utf8 = str::from_utf8(&data)?;
            json::from_str(utf8)?
        };

        // Discard deserializing errors (too verbose) for a new one.
        let manifests = res.or_else(|_| Err(PullError::InvalidData))?;

        let cache = config.available();

        // Erase old cache
        cache.erase_repository(&self.repository)?;

        // Update cache
        let len = manifests.len();
        for (i, manifest) in manifests.into_iter().enumerate() {
            let package = Package::from(&self.repository, manifest);
            cache
                .update(&package)
                .with_context(|_| PullError::CantUpdateCache(package.id().to_string()))?;
            notifier.notify(self, Notification::Progress(i + 1, len));
        }
        Ok(())
    }
}

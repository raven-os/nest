/// The "pull" transaction
use std::io::{Cursor, Seek, Write};
use std::str;

use failure::{Error, ResultExt};
use serde_json;

use crate::cache::CacheErrorKind;
use crate::lock_file::LockFileOwnership;
use crate::package::PackageManifest;
use crate::repository::Repository;

/// Structure representing a "pull" transaction
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct PullTransaction<'a, 'b> {
    target_repository: Repository<'a, 'b>,
    data: Vec<u8>,
}

impl<'a, 'b> PullTransaction<'a, 'b> {
    /// Creates a "pull" transaction for a given [`Repository`]
    pub fn from(repository: Repository<'a, 'b>) -> Self {
        PullTransaction {
            target_repository: repository,
            data: Vec::new(),
        }
    }

    /// Returns the target [`Repository`] for this transaction
    pub fn target_repository(&self) -> &Repository<'a, 'b> {
        &self.target_repository
    }

    /// Returns a writer to store data
    pub fn writer(&mut self) -> impl Write + Seek + '_ {
        Cursor::new(&mut self.data)
    }

    /// Save the stored data to the available packages cache
    pub fn save_to_cache(
        &self,
        config: &crate::config::Config,
        ownership: &LockFileOwnership,
    ) -> Result<(), Error> {
        let res: Result<Vec<PackageManifest>, Error> = try {
            let utf8 = str::from_utf8(&self.data)?;
            serde_json::from_str(utf8)?
        };

        let manifests = res.context(CacheErrorKind::CacheWriteError)?;
        let cache = config.available_packages_cache(ownership);

        cache.erase_repository(&self.target_repository)?;

        for manifest in manifests {
            cache
                .update(&manifest)
                .with_context(|_| manifest.name().to_string())
                .context(CacheErrorKind::CacheWriteError)?;
        }
        Ok(())
    }
}

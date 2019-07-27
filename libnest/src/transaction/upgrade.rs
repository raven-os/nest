use failure::Error;

use crate::config::Config;
use crate::lock_file::LockFileOwnership;
use crate::package::{NPFExplorer, PackageID};

use super::download::PackageDownload;
use super::extract::extract_package;
use super::remove::remove_package;
use super::{InstallError, InstallErrorKind::*, RemoveError, RemoveErrorKind::*};

/// Structure representing an upgrade transaction
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct UpgradeTransaction {
    old: PackageID,
    new: PackageID,
}

impl UpgradeTransaction {
    /// Creates an [`UpgradeTransaction`] from an old [`PackageID`] and a new [`PackageID`]
    pub fn from(old: PackageID, new: PackageID) -> Self {
        UpgradeTransaction { old, new }
    }

    /// Retrieves a reference over the old target package for this transaction
    pub fn old_target(&self) -> &PackageID {
        &self.old
    }

    /// Retrieves a reference over the new target package for this transaction
    pub fn new_target(&self) -> &PackageID {
        &self.new
    }

    /// Get the download associated to this transaction
    pub fn associated_download(&self) -> PackageDownload {
        PackageDownload::from(self.new_target().clone())
    }

    fn remove_old_package(
        &self,
        config: &Config,
        lock_ownership: &LockFileOwnership,
    ) -> Result<(), RemoveError> {
        let npf_path = config
            .paths()
            .downloaded()
            .join(self.old_target().repository().as_str())
            .join(self.old_target().category().as_str())
            .join(self.old_target().name().as_str())
            .join(format!(
                "{}-{}.nest",
                self.old_target().name(),
                self.old_target().version()
            ));

        let npf_explorer = NPFExplorer::from(&npf_path).map_err(|_| InvalidCachedPackageFile)?;

        remove_package(config, lock_ownership, npf_explorer, self.old_target())
    }

    fn install_new_package(
        &self,
        config: &Config,
        lock_ownership: &LockFileOwnership,
    ) -> Result<(), InstallError> {
        let npf_path = config
            .paths()
            .downloaded()
            .join(self.new_target().repository().as_str())
            .join(self.new_target().category().as_str())
            .join(self.new_target().name().as_str())
            .join(format!(
                "{}-{}.nest",
                self.new_target().name(),
                self.new_target().version()
            ));

        let npf_explorer = NPFExplorer::from(&npf_path).map_err(|_| InvalidPackageFile)?;

        extract_package(config, lock_ownership, npf_explorer, self.new_target())
    }

    /// Perform the upgrade transaction
    pub fn perform(
        &self,
        config: &Config,
        lock_ownership: &LockFileOwnership,
    ) -> Result<(), Error> {
        self.remove_old_package(config, lock_ownership)?;
        self.install_new_package(config, lock_ownership)?;

        Ok(())
    }
}

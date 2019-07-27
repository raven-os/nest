use crate::config::Config;
use crate::lock_file::LockFileOwnership;
use crate::package::PackageID;

use super::download::PackageDownload;
use super::extract::extract_package;
use super::{InstallError, InstallErrorKind::*};

/// Structure representing an "install" transaction
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct InstallTransaction {
    target: PackageID,
}

impl InstallTransaction {
    /// Creates an [`InstallTransaction`] from a given [`PackageID`]
    #[inline]
    pub fn from(target: PackageID) -> Self {
        InstallTransaction { target }
    }

    /// Returns the target [`PackageID`] for this transaction
    pub fn target(&self) -> &PackageID {
        &self.target
    }

    /// Create a download associated to this transaction
    pub fn associated_download(&self) -> PackageDownload {
        PackageDownload::from(self.target().clone())
    }

    /// Extracts the downloaded file and performs the installation
    pub fn extract(
        &self,
        config: &Config,
        lock_ownership: &LockFileOwnership,
    ) -> Result<(), InstallError> {
        let downloaded_packages = config.downloaded_packages_cache(lock_ownership);
        let npf_explorer = downloaded_packages
            .explore_package(self.target())
            .map_err(|_| InvalidPackageFile)?;

        extract_package(config, lock_ownership, npf_explorer, self.target())
    }
}

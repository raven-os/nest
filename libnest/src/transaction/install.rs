use crate::package::PackageID;

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
}

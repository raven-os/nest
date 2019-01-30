use crate::package::PackageID;

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
}

use crate::package::PackageID;

/// Structure representing a "remove" transaction
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct RemoveTransaction {
    target: PackageID,
}

impl RemoveTransaction {
    /// Creates a [`RemoveTransaction`] from a given [`PackageID`]
    pub fn from(target: PackageID) -> Self {
        RemoveTransaction { target }
    }
}

use std::fmt::{self, Display, Formatter};

use serde_derive::{Serialize, Deserialize};

use crate::cache::depgraph::NodeId;
use crate::package::PackageRequirement;

/// The kind of a node's requirement.
///
/// A node can require any kind of node: a group, or a package.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum RequirementKind {
    /// The node requires a groue
    Group {
        /// The name of the required group
        name: String,
    },
    /// The node requires a package
    Package {
        /// The [`PackageRequirement`] that the package must match.
        package_req: PackageRequirement,
    },
}

impl Display for RequirementKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            RequirementKind::Group { name, .. } => write!(f, "@{}", name),
            RequirementKind::Package { package_req, .. } => write!(f, "{}", package_req),
        }
    }
}

/// A node's requirement. It wraps a [`NodeRequirementKind`] and the [`NodeId`] of the
/// [`Node`] that fulfills this requirement.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Requirement {
    kind: RequirementKind,
    fulfiller: NodeId,
    fulfilled: NodeId,
}

impl Requirement {
    #[inline]
    pub(crate) fn from(kind: RequirementKind, fulfiller: NodeId, fulfilled: NodeId) -> Requirement {
        Requirement {
            kind,
            fulfiller,
            fulfilled,
        }
    }

    /// Returns a reference to the kind of this requirement.
    #[inline]
    pub fn kind(&self) -> &RequirementKind {
        &self.kind
    }

    /// Returns the [`NodeId`] of the [`Node`] that fulfills this requirement.
    #[inline]
    pub fn fulfiller(&self) -> NodeId {
        self.fulfiller
    }

    /// Returns the [`NodeId`] of the [`Node`] that is fulfilled by this requirement.
    #[inline]
    pub fn fulfilled(&self) -> NodeId {
        self.fulfilled
    }
}

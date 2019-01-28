use serde_derive::{Deserialize, Serialize};

use crate::package::PackageRequirement;

use super::{GroupName, NodeID};

/// Type representing unique identifiers of a requirement in the dependency graph
pub type RequirementID = usize;

/// The kind of a node's requirement.
///
/// A node can hold a requirement to any kind of node: a group, or a package.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum RequirementKind {
    /// The node requires a group
    Group {
        /// The name of the required group
        name: GroupName,
    },
    /// The node requires a package
    Package {
        /// The [`PackageRequirement`] that the package must match.
        package_req: PackageRequirement,
    },
}

impl std::fmt::Display for RequirementKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RequirementKind::Group { name, .. } => write!(f, "{}", name.as_str()),
            RequirementKind::Package { package_req, .. } => write!(f, "{}", package_req),
        }
    }
}

/// A node's requirement. It wraps a [`NodeRequirementKind`] and the [`NodeID`] of the
/// [`Node`] that fulfills this requirement.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Requirement {
    kind: RequirementKind,
    fulfilling: Option<NodeID>,
    fulfilled: NodeID,
}

impl Requirement {
    #[allow(dead_code)] // TODO: Remove this when the function is used
    #[inline]
    pub(crate) fn from(
        kind: RequirementKind,
        fulfilling: Option<NodeID>,
        fulfilled: NodeID,
    ) -> Requirement {
        Requirement {
            kind,
            fulfilling,
            fulfilled,
        }
    }

    /// Returns a reference to the kind of this requirement.
    #[inline]
    pub fn kind(&self) -> &RequirementKind {
        &self.kind
    }

    /// Returns the [`NodeID`] of the [`Node`] that fulfills this requirement.
    #[inline]
    pub fn fulfilling_node(&self) -> Option<NodeID> {
        self.fulfilling
    }

    /// Returns the [`NodeID`] of the [`Node`] that is fulfilled by this requirement.
    #[inline]
    pub fn fulfilled_node(&self) -> NodeID {
        self.fulfilled
    }
}

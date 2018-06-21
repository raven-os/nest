use cache::depgraph::NodeId;
use package::PackageRequirement;

/// The kind of a node's requirement.
///
/// A node can require any kind of node: a group, or a package.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum NodeRequirementKind {
    /// The node requires a groue
    Group {
        /// The name of the required group
        name: String,
    },
    /// The node requires a package
    Package {
        /// The [`PackageRequirement`] that the package must match.
        requirement: PackageRequirement,
    },
}

/// A node's requirement. It wraps a [`NodeRequirementKind`] and the [`NodeId`] of the
/// [`Node`] that fulfills this requirement.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct NodeRequirement {
    kind: NodeRequirementKind,
    fulfiller: NodeId,
}

impl NodeRequirement {
    #[inline]
    pub(crate) fn from(kind: NodeRequirementKind, fulfiller: NodeId) -> NodeRequirement {
        NodeRequirement { kind, fulfiller }
    }

    /// Returns a reference to the kind of this requirement.
    #[inline]
    pub fn kind(&self) -> &NodeRequirementKind {
        &self.kind
    }

    /// Returns the [`NodeId`] of the [`Node`] that fulfills this requirement.
    #[inline]
    pub fn fulfiller(&self) -> NodeId {
        self.fulfiller
    }
}

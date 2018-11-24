use crate::cache::depgraph::{DependencyGraph, NodeId, NodeKind};
use crate::package::PackageRequirement;

/// A query on the [`DependencyGraph`][1].
///
/// This handle takes a [`PackageRequirement] and will look into the [`DependencyGraph`] to find all [`Node`]s
/// matching the given PackageRequirement.
///
/// [1]: struct.DependencyGraph.html
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct DependencyGraphQuery<'a, 'b> {
    depgraph: &'a DependencyGraph,
    package_req: &'b PackageRequirement,
}

impl<'a, 'b> DependencyGraphQuery<'a, 'b> {
    #[inline]
    pub(crate) fn from(
        depgraph: &'a DependencyGraph,
        package_req: &'b PackageRequirement,
    ) -> DependencyGraphQuery<'a, 'b> {
        DependencyGraphQuery {
            depgraph,
            package_req,
        }
    }

    /// Performs the search, returning a vector of [`NodeId`] matching the [`PackageRequirement`] of this query.
    #[inline]
    pub fn perform(&self) -> Vec<NodeId> {
        let mut results = Vec::new();

        for (node_id, node) in &self.depgraph.nodes {
            if let NodeKind::Package { id, .. } = &node.kind {
                if self.package_req.matches(&id) {
                    results.push(*node_id);
                }
            }
        }
        results
    }
}

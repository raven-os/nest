use std::collections::HashSet;

use failure::Error;

use cache::depgraph::{DependencyGraph, NodeId, NodeKind, ROOT_ID};
use transaction::{Install, Remove, Transaction};

/// Stores intermediate results used when calculating the differences between two related [`DependencyGraph`].
#[derive(Debug, Default)]
pub struct DependencyGraphDiff {
    taint: HashSet<NodeId>,
    transactions: Vec<Box<Transaction>>,
}

impl DependencyGraphDiff {
    /// Creates a new [`DependencyGraphDiff`], ready to do some calculations.
    pub fn new() -> DependencyGraphDiff {
        DependencyGraphDiff {
            taint: HashSet::new(),
            transactions: Vec::new(),
        }
    }

    fn diff_nodes(
        &mut self,
        left_graph: &DependencyGraph,
        right_graph: &DependencyGraph,
        node_id: NodeId,
    ) -> Result<(), Error> {
        // Do not treat nodes that has already been treated.
        if self.taint.contains(&node_id) {
            return Ok(());
        }
        self.taint.insert(node_id);
        match (
            left_graph.nodes.get(&node_id),
            right_graph.nodes.get(&node_id),
        ) {
            (Some(left_node), None) => {
                // Removed nodes
                // Create a transaction only if the node is a package
                if let NodeKind::Package { id, .. } = &left_node.kind {
                    self.transactions.push(Box::new(Remove::from(id.clone())));
                }
                // Repeat on dependencies
                for requirement_id in &left_node.requirements {
                    self.diff_nodes(
                        left_graph,
                        right_graph,
                        left_graph.requirements[requirement_id].fulfiller(),
                    )?;
                }
            }
            (None, Some(right_node)) => {
                // Added nodes
                // Repeat on dependencies
                for requirement_id in &right_node.requirements {
                    self.diff_nodes(
                        left_graph,
                        right_graph,
                        right_graph.requirements[requirement_id].fulfiller(),
                    )?;
                }
                // Create a transaction only if the node is a package
                if let NodeKind::Package { id, .. } = &right_node.kind {
                    self.transactions.push(Box::new(Install::from(id.clone())));
                }
            }
            (Some(left_node), Some(right_node)) => {
                // Present in both graph (maybe update, maybe nothing)
                // TODO Test if upgrade/downgrad/reinstall, and add a transaction if it's the case.
                // Repeat on dependencies
                for requirement_id in &left_node.requirements {
                    self.diff_nodes(
                        left_graph,
                        right_graph,
                        left_graph.requirements[requirement_id].fulfiller(),
                    )?;
                }
                for requirement_id in &right_node.requirements {
                    self.diff_nodes(
                        left_graph,
                        right_graph,
                        right_graph.requirements[requirement_id].fulfiller(),
                    )?;
                }
            }
            _ => (),
        }
        Ok(())
    }

    /// Calculates the transactions needed to go from the `left_graph` to the `right_graph`.
    ///
    /// The graph *MUST* be related, or the behaviour of this function is unspecified.
    /// This means that the `right_graph` must be the evolution of the left_graph. That it is the result
    /// of multiple additions, removal or modification of requirements.
    pub fn perform(
        mut self,
        left_graph: &DependencyGraph,
        right_graph: &DependencyGraph,
    ) -> Result<Vec<Box<Transaction>>, Error> {
        self.diff_nodes(left_graph, right_graph, ROOT_ID)?;
        Ok(self.transactions)
    }
}

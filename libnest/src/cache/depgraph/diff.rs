use std::collections::HashSet;

use crate::transaction::{InstallTransaction, RemoveTransaction, Transaction, UpgradeTransaction};

use super::{node::ROOT_ID, DependencyGraph, NodeID, NodeKind};

/// Structure used to calculate differences between two related [`DependencyGraph`]s
#[derive(Clone, Debug, Default)]
pub struct DependencyGraphDiff;

impl DependencyGraphDiff {
    /// Creates a new [`DependencyGraphDiff`]
    pub fn new() -> Self {
        DependencyGraphDiff {}
    }

    fn diff_nodes<'a, 'b>(
        &self,
        transactions: &mut Vec<Transaction<'a, 'b>>,
        visited: &mut HashSet<NodeID>,
        old_graph: &DependencyGraph,
        new_graph: &DependencyGraph,
        node_id: NodeID,
    ) {
        if visited.contains(&node_id) {
            return;
        }
        visited.insert(node_id);

        match (
            old_graph.nodes().get(&node_id),
            new_graph.nodes().get(&node_id),
        ) {
            (Some(left_node), None) => {
                // The node is found in the old graph but not in the new one

                // Produce a Remove, if and only if it is a package
                if let NodeKind::Package { id, .. } = left_node.kind() {
                    transactions.push(Transaction::Remove(RemoveTransaction::from(id.clone())));
                }

                // Continue the diff on the requirements
                for requirement_id in left_node.requirements().iter() {
                    self.diff_nodes(
                        transactions,
                        visited,
                        old_graph,
                        new_graph,
                        old_graph
                            .requirements()
                            .get(requirement_id)
                            // safe-to-use because we know the node is in the graph
                            .unwrap()
                            .fulfilling_node_id()
                            // safe-to-use because we know the graph is solved, thus each requirement is fulfilled
                            .unwrap(),
                    );
                }
            }
            (None, Some(right_node)) => {
                // The node is found in the new graph but not in the old one

                // Repeat on dependencies
                for requirement_id in right_node.requirements().iter() {
                    self.diff_nodes(
                        transactions,
                        visited,
                        old_graph,
                        new_graph,
                        new_graph
                            .requirements()
                            .get(requirement_id)
                            // safe-to-use because we know the node is in the graph
                            .unwrap()
                            .fulfilling_node_id()
                            // safe-to-use because we know the graph is solved, thus each requirement is fulfilled
                            .unwrap(),
                    );
                }

                // Produce an Install if and only if the node is a package
                if let NodeKind::Package { id, .. } = right_node.kind() {
                    transactions.push(Transaction::Install(InstallTransaction::from(id.clone())));
                }
            }
            (Some(left_node), Some(right_node)) => {
                // The node is found in both graphs

                // Repeat on dependencies
                for requirement_id in left_node.requirements().iter() {
                    self.diff_nodes(
                        transactions,
                        visited,
                        old_graph,
                        new_graph,
                        old_graph
                            .requirements()
                            .get(requirement_id)
                            // safe-to-use because we know the node is in the graph
                            .unwrap()
                            .fulfilling_node_id()
                            // safe-to-use because we know the graph is solved, thus each requirement is fulfilled
                            .unwrap(),
                    );
                }
                for requirement_id in right_node.requirements().iter() {
                    self.diff_nodes(
                        transactions,
                        visited,
                        old_graph,
                        new_graph,
                        new_graph
                            .requirements()
                            .get(requirement_id)
                            // safe-to-use because we know the node is in the graph
                            .unwrap()
                            .fulfilling_node_id()
                            // safe-to-use because we know the graph is solved, thus each requirement is fulfilled
                            .unwrap(),
                    );
                }

                // Test whether versions differ, and add a transaction
                if let (
                    NodeKind::Package { id: id_left, .. },
                    NodeKind::Package { id: id_right, .. },
                ) = (left_node.kind(), right_node.kind())
                {
                    if id_left.version() != id_right.version() {
                        transactions.push(Transaction::Upgrade(UpgradeTransaction::from(
                            id_left.clone(),
                            id_right.clone(),
                        )));
                    }
                }
            }
            _ => (),
        }
    }

    /// Performs a diff between two solved graphs
    /// The result of the diff is a vector of [`Transactions`] required in order to transition
    /// from the old graph to the new graph
    pub fn perform<'a, 'b>(
        &self,
        old_graph: &DependencyGraph,
        new_graph: &DependencyGraph,
    ) -> Vec<Transaction<'a, 'b>> {
        let mut transactions = Vec::new();

        self.diff_nodes(
            &mut transactions,
            &mut HashSet::new(),
            old_graph,
            new_graph,
            ROOT_ID,
        );
        transactions
    }
}

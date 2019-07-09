use std::collections::HashSet;

use crate::transaction::{InstallTransaction, RemoveTransaction, Transaction, UpgradeTransaction};

use super::{DependencyGraph, GroupName, NodeKind, NodeName};

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
        visited: &mut HashSet<NodeName>,
        old_graph: &DependencyGraph,
        new_graph: &DependencyGraph,
        node_name: NodeName,
    ) {
        if visited.contains(&node_name) {
            return;
        }
        visited.insert(node_name.clone());

        match (
            old_graph
                .node_names()
                .get(&node_name)
                .and_then(|id| old_graph.nodes().get(id)),
            new_graph
                .node_names()
                .get(&node_name)
                .and_then(|id| new_graph.nodes().get(id)),
        ) {
            (Some(left_node), None) => {
                // The node is found in the old graph but not in the new one

                // Produce a Remove, if and only if it is a package
                if let NodeKind::Package { id, .. } = left_node.kind() {
                    transactions.push(Transaction::Remove(RemoveTransaction::from(id.clone())));
                }

                // Continue the diff on the requirements
                for requirement_id in left_node.requirements().iter() {
                    let node_id = old_graph
                        .requirements()
                        .get(requirement_id)
                        // safe-to-use because we know the node is in the graph
                        .unwrap()
                        .fulfilling_node_id()
                        // safe-to-use because we know the graph is solved, thus each requirement is fulfilled
                        .unwrap();
                    let node = old_graph.nodes().get(&node_id).unwrap();

                    self.diff_nodes(
                        transactions,
                        visited,
                        old_graph,
                        new_graph,
                        NodeName::from(node.kind().clone()),
                    );
                }
            }
            (None, Some(right_node)) => {
                // The node is found in the new graph but not in the old one

                // Repeat on dependencies
                for requirement_id in right_node.requirements().iter() {
                    let node_id = new_graph
                        .requirements()
                        .get(requirement_id)
                        // safe-to-use because we know the node is in the graph
                        .unwrap()
                        .fulfilling_node_id()
                        // safe-to-use because we know the graph is solved, thus each requirement is fulfilled
                        .unwrap();
                    let node = new_graph.nodes().get(&node_id).unwrap();

                    self.diff_nodes(
                        transactions,
                        visited,
                        old_graph,
                        new_graph,
                        NodeName::from(node.kind().clone()),
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
                    let node_id = old_graph
                        .requirements()
                        .get(requirement_id)
                        // safe-to-use because we know the node is in the graph
                        .unwrap()
                        .fulfilling_node_id()
                        // safe-to-use because we know the graph is solved, thus each requirement is fulfilled
                        .unwrap();
                    let node = old_graph.nodes().get(&node_id).unwrap();

                    self.diff_nodes(
                        transactions,
                        visited,
                        old_graph,
                        new_graph,
                        NodeName::from(node.kind().clone()),
                    );
                }
                for requirement_id in right_node.requirements().iter() {
                    let node_id = new_graph
                        .requirements()
                        .get(requirement_id)
                        // safe-to-use because we know the node is in the graph
                        .unwrap()
                        .fulfilling_node_id()
                        // safe-to-use because we know the graph is solved, thus each requirement is fulfilled
                        .unwrap();
                    let node = new_graph.nodes().get(&node_id).unwrap();

                    self.diff_nodes(
                        transactions,
                        visited,
                        old_graph,
                        new_graph,
                        NodeName::from(node.kind().clone()),
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
            _ => unreachable!(),
        }
    }

    /// Performs a diff between two solved graphs
    /// The result of the diff is a vector of [`Transactions`] required in order to transition
    /// from the old graph to the new graph.
    ///
    /// The resulting transactions are ordered in a way that ensures a valid system state if they
    /// are applied (installations of dependencies come before installations of dependents, etc)
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
            NodeName::Group(GroupName::root_group()),
        );
        transactions
    }
}

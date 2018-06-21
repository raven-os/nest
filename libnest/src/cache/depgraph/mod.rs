//! Handles and methods to process the dependency graph.

mod query;
mod requirement;
pub use self::query::DependencyGraphQuery;
pub use self::requirement::{NodeRequirement, NodeRequirementKind};

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::ops::Sub;
use std::path::Path;

use failure::{Error, ResultExt};
use json;

use config::Config;
use error::DepGraphErrorKind;
use package::{PackageId, PackageRequirement};
use transaction::{Install, Transaction};

/// The unique identifier of a node of the dependency graph.
pub type NodeId = usize;

static ROOT_ID: NodeId = 0;

/// The kind of a node.
///
/// A node is not necessarily a package, it can also be a group: a named list of requirements.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum NodeKind {
    /// The node is a group
    Group {
        /// The name of the group
        name: String,
    },
    /// The node is a package
    Package {
        /// The [`PackageId`] of this node.
        id: PackageId,
    },
}

/// A node of the dependency graph.
///
/// A node is represented by a content (a [`NodeKind`][1]) and a list of [`NodeRequirement`][2] that must
/// be satisfied for the graph to be valid.
///
/// [1]: enum.NodeKind.html
/// [2]: requirement/struct.NodeRequirement.html
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Node {
    kind: NodeKind,
    requirements: Vec<NodeRequirement>,
}

impl Node {
    #[inline]
    fn new(kind: NodeKind) -> Node {
        Node {
            kind,
            requirements: Vec::new(),
        }
    }

    /// Returns the [`NodeKind`][1] of this node.
    ///
    /// [1]: enum.NodeKind.html
    #[inline]
    pub fn kind(&self) -> &NodeKind {
        &self.kind
    }

    /// Returns a reference on a [`Vec`][1]<[`NodeRequirement`][2]> this node depends on.
    ///
    /// [1]: https://doc.rust-lang.org/std/vec/struct.Vec.html
    /// [2]: requirement/struct.NodeRequirement.html
    #[inline]
    pub fn requirements(&self) -> &Vec<NodeRequirement> {
        &self.requirements
    }
}

/// The dependency graph: a serializable collection of [`Node`][1].
///
/// [1]: struct.Node.html
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct DependencyGraph {
    next_id: usize,
    nodes: HashMap<NodeId, Node>,
}

impl DependencyGraph {
    #[inline]
    pub(crate) fn new() -> DependencyGraph {
        let mut nodes = HashMap::new();

        nodes.insert(
            ROOT_ID,
            Node::new(NodeKind::Group {
                name: String::from("@root"),
            }),
        );
        DependencyGraph {
            next_id: ROOT_ID + 1,
            nodes,
        }
    }

    #[inline]
    pub(crate) fn load<P: AsRef<Path>>(path: P) -> Result<DependencyGraph, Error> {
        let path = path.as_ref();
        if path.exists() {
            let file = File::open(path).with_context(|_| path.display().to_string())?;
            Ok(json::from_reader(&file).with_context(|_| path.display().to_string())?)
        } else {
            Ok(DependencyGraph::new())
        }
    }

    /// Saves the dependency graph to the location stored in the given [`Config`][1].
    ///
    /// [1]: ../../config/struct.Config.html
    #[inline]
    pub fn save(&self, config: &Config) -> Result<(), Error> {
        let path = config.paths().depgraph();
        let mut file = File::create(path).with_context(|_| path.display().to_string())?;
        json::to_writer_pretty(&file, self).with_context(|_| path.display().to_string())?;
        writeln!(file)?;
        Ok(())
    }

    #[inline]
    fn next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// Returns the id of the root [`Node`][1].
    ///
    /// [1]: struct.Node.html
    #[inline]
    pub fn root_id(&self) -> NodeId {
        ROOT_ID
    }

    /// Returns a reference on the [`Node`][1] with the given [`NodeId`][2], or `None` if it wasn't found.
    ///
    /// [1]: struct.Node.html
    /// [2]: type.NodeId.html
    #[inline]
    pub fn node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(&id)
    }

    /// Returns a mutable reference on the [`Node`][1] with the given [`NodeId`][2], or `None` if it wasn't found.
    ///
    /// [1]: struct.Node.html
    /// [2]: type.NodeId.html
    #[inline]
    pub fn node_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(&id)
    }

    /// Returns a [`HashMap`][1]<[`NodeId`], [`Node`]> of all [`Node`][3]s and it's [`NodeId`][2] currently in the dependency graph.
    ///
    /// [1]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
    /// [2]: type.NodeId.html
    /// [3]: struct.Node.html
    #[inline]
    pub fn nodes(&self) -> &HashMap<NodeId, Node> {
        &self.nodes
    }

    /// Returns a handle to perform a search on the dependency graph following the given package requirement.
    #[inline]
    pub fn search<'a, 'b>(
        &'a self,
        package_req: &'b PackageRequirement,
    ) -> DependencyGraphQuery<'a, 'b> {
        DependencyGraphQuery::from(self, package_req)
    }

    /// Adds the given [`PackgeRequirement`] to the [`DependencyGraph`], as a child of the given node (represented by it's [`NodeId`]).
    pub fn add_package(
        &mut self,
        config: &Config,
        parent: NodeId,
        requirement: &PackageRequirement,
    ) -> Result<(), Error> {
        // First, check if their isn't already a package that matches the requirement
        let packages = self.search(requirement).perform();
        if !packages.is_empty() {
            return Ok(());
        }

        // Look for a package in the cache that matches the requirement
        let results = {
            let query = config.available().search(requirement);
            query.perform()?
        };
        let package = results
            .get(0)
            .ok_or_else(|| DepGraphErrorKind::CantFindPackage(requirement.to_string()))?;

        // Create and insert the child node
        let child_id = self.next_id();
        {
            let child_node = Node::new(NodeKind::Package { id: package.id() });

            self.nodes.insert(child_id, child_node);
        }

        // Create the connexion with the parent node
        {
            let parent = self
                .node_mut(parent)
                .ok_or(DepGraphErrorKind::InvalidNodeId)
                .with_context(|_| parent.to_string())?;

            let req = NodeRequirement::from(
                NodeRequirementKind::Package {
                    requirement: requirement.clone(),
                },
                child_id,
            );
            parent.requirements.push(req);
        }

        for (name, req) in package.manifest().dependencies() {
            self.add_package(
                config,
                child_id,
                &PackageRequirement::from(name, req.clone()),
            )?;
        }
        Ok(())
    }
}

// XXX: The implementation of `sub` kind of sucks, but i don't have any immediate better ideas,
// so if you have, feel free to improve.
impl Sub for DependencyGraph {
    type Output = Vec<Box<Transaction>>;

    fn sub(self, other: DependencyGraph) -> Self::Output {
        let mut out = Vec::new();
        let left_root = &self.nodes[&self.root_id()];
        let right_root = &other.nodes[&self.root_id()];
        diff_common_nodes(&mut out, &self, &other, left_root, right_root);
        out
    }
}

fn diff_added_nodes(
    v: &mut Vec<Box<Transaction>>,
    left_graph: &DependencyGraph,
    right_graph: &DependencyGraph,
    node: &Node,
) {
    // First handle requirements
    for requirement in node.requirements() {
        let fulfiller_id = requirement.fulfiller();
        let right_node = &right_graph.nodes[&fulfiller_id];
        if let Some(left_node) = left_graph.nodes.get(&fulfiller_id) {
            diff_common_nodes(v, left_graph, right_graph, left_node, right_node);
        } else {
            diff_added_nodes(v, left_graph, right_graph, right_node);
        }
    }
    // Then add a new transaction if the node is a package
    if let NodeKind::Package { id, .. } = node.kind() {
        v.push(Box::new(Install::from(id.clone())));
    }
}

fn diff_removed_nodes(
    v: &mut Vec<Box<Transaction>>,
    left_graph: &DependencyGraph,
    right_graph: &DependencyGraph,
    node: &Node,
) {
    // First handle requirements
    for requirement in node.requirements() {
        let fulfiller_id = requirement.fulfiller();
        let left_node = &left_graph.nodes[&fulfiller_id];
        if let Some(right_node) = right_graph.nodes.get(&fulfiller_id) {
            diff_common_nodes(v, left_graph, right_graph, left_node, right_node);
        } else {
            diff_removed_nodes(v, left_graph, right_graph, left_node);
        }
    }
    // Then add a new transaction if the node is a package
    if let NodeKind::Package { .. } = node.kind() {
        // TODO: Push a Remove transaction
        // v.push(Box::new(Remove::new(id.clone())));
    }
}

fn diff_common_nodes(
    v: &mut Vec<Box<Transaction>>,
    left_graph: &DependencyGraph,
    right_graph: &DependencyGraph,
    left_node: &Node,
    right_node: &Node,
) {
    for requirement in left_node.requirements() {
        let fulfiller_id = requirement.fulfiller();
        let left_node = &left_graph.nodes[&fulfiller_id];

        // Test if this node is present in the left graph but not in the right graph (REMOVED)
        // If this node is present in both graph, then diff it's representation on both graphs.
        if let Some(right_node) = right_graph.nodes.get(&fulfiller_id) {
            diff_common_nodes(v, left_graph, right_graph, left_node, right_node);
        } else {
            diff_removed_nodes(v, left_graph, right_graph, left_node);
        }
    }

    for requirement in right_node.requirements() {
        let fulfiller_id = requirement.fulfiller();
        let new_node = &right_graph.nodes[&fulfiller_id];

        // Test if this node is present in the righ graph but not in the left graph (ADDED)
        if left_graph.nodes.get(&fulfiller_id).is_none() {
            diff_added_nodes(v, left_graph, right_graph, new_node);
        }
    }
}

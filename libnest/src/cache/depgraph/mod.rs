//! Handles and methods to process the dependency graph.

mod diff;
mod query;
mod requirement;
pub use self::diff::DependencyGraphDiff;
pub use self::query::DependencyGraphQuery;
pub use self::requirement::{Requirement, RequirementKind};

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::path::Path;

use failure::{Error, ResultExt};
use json;

use config::Config;
use error::DepGraphErrorKind;
use package::{PackageId, PackageRequirement};

/// The unique identifier of a node of the dependency graph.
pub type NodeId = usize;

/// The unique identifier of a requirement of the dependency graph.
pub type RequirementId = usize;

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
/// A node is represented by a content (a [`NodeKind`][1]), a list of [`NodeRequirement`][2] that must
/// be satisfied for the graph to be valid, and a list of requirements that other nodes have on this one.
///
/// [1]: enum.NodeKind.html
/// [2]: requirement/struct.NodeRequirement.html
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
struct Node {
    kind: NodeKind,
    requirements: HashSet<RequirementId>,
    dependents: HashSet<RequirementId>,
}

impl Node {
    #[inline]
    fn new(kind: NodeKind) -> Node {
        Node {
            kind,
            requirements: HashSet::new(),
            dependents: HashSet::new(),
        }
    }
}

/// The dependency graph: a serializable collection of [`Node`][1].
///
/// [1]: struct.Node.html
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct DependencyGraph {
    next_node_id: usize,
    next_requirement_id: usize,
    nodes: HashMap<NodeId, Node>,
    requirements: HashMap<RequirementId, Requirement>,
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
            next_node_id: ROOT_ID + 1,
            next_requirement_id: 0,
            nodes,
            requirements: HashMap::new(),
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
    fn next_node_id(&mut self) -> usize {
        let id = self.next_node_id;
        self.next_node_id += 1;
        id
    }

    #[inline]
    fn next_requirement_id(&mut self) -> usize {
        let id = self.next_requirement_id;
        self.next_requirement_id += 1;
        id
    }

    /// Returns the id of the root [`Node`][1].
    ///
    /// [1]: struct.Node.html
    #[inline]
    pub fn root_id(&self) -> NodeId {
        ROOT_ID
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
    ///
    /// In case of error, the [`DependencyGraph`] is left in an unspecified state, possibly unstable, and shouldn't be used nor saved anymore.
    pub fn add_package(
        &mut self,
        config: &Config,
        parent_id: NodeId,
        package_req: PackageRequirement,
    ) -> Result<(), Error> {
        let manifest;
        let child_id;

        // First, check if their isn't already a package that matches the requirement
        let node_ids = self.search(&package_req).perform();
        if node_ids.is_empty() {
            // If this requirement isn't satisfied yet, find the package and add it as a new node.
            // Look for a package in the cache that matches the requirement
            let mut results = config.available().search(&package_req).perform()?;

            let package = {
                if results.len() == 1 {
                    results.remove(0)
                } else if results.is_empty() {
                    Err(DepGraphErrorKind::CantFindPackage(package_req.to_string()))?;
                    unreachable!()
                } else {
                    Err(DepGraphErrorKind::ImpreciseRequirement(
                        package_req.to_string(),
                    ))?;
                    unreachable!()
                }
            };
            // Generate an id for the child node
            child_id = self.next_node_id();
            self.nodes
                .insert(child_id, Node::new(NodeKind::Package { id: package.id() }));
            manifest = Some(package.manifest().clone());
        } else {
            // The requirement is already satisfied by an other node: let's find it.
            // We'll take the first one that matches our requirement (that's debatable, actually. We should probably be a bit more picky)
            child_id = node_ids[0];
            manifest = None;
        }

        // From now on, the child node represent's either a new node freshly added (new package) or an existing node (like an already existing dependency).
        // In either cases, we want to link the parent node with the child node.

        // Generate an id for the requirement that will link together the parent and the child node.
        let requirement_id = self.next_requirement_id();

        // Create the requirement and insert it.
        self.requirements.insert(
            requirement_id,
            Requirement::from(
                RequirementKind::Package { package_req },
                child_id,
                parent_id,
            ),
        );

        // Add this requirement as a dependency of the parent node.
        self.nodes
            .get_mut(&parent_id)
            .expect("invalid parent node id")
            .requirements
            .insert(requirement_id);

        self.nodes
            .get_mut(&child_id)
            .expect("invalid child node id")
            .dependents
            .insert(requirement_id);

        // Repeat for all dependencies only if this is a new package (not needed otherwise).
        if let Some(manifest) = manifest {
            for (name, req) in manifest.dependencies() {
                self.add_package(
                    config,
                    child_id,
                    PackageRequirement::from(name, req.clone()),
                )?;
            }
        }
        Ok(())
    }

    /// Removes the given [`PackgeRequirement`] of the [`DependencyGraph`] if it's a child of the given node (represented by it's [`NodeId`]).
    pub fn remove_requirement(
        &mut self,
        parent_id: NodeId,
        target_requirement: &RequirementKind,
    ) -> Result<(), Error> {
        let requirement_id = self
            .nodes
            .get(&parent_id)
            .expect("invalid parent node id")
            .requirements
            .iter()
            .find(|requirement_id| {
                // This is unfortunately ugly, but it looks like `RequirementKind` fails to implement `Eq` correctly.
                // I'm not sure of that yet, nor how it fails, and will investigate soon.
                // This `to_string()` serves as a temporary mesure.
                self.requirements[&requirement_id].kind().to_string()
                    == target_requirement.to_string()
            })
            .cloned()
            .ok_or_else(|| DepGraphErrorKind::UnknownRequirement(target_requirement.to_string()))?;

        // We want to remove the child node only if this requirement was it's last bound to the DependencyGraph.
        // If it's the case, we also want to repeat this recursively.

        let child_id = self.requirements[&requirement_id].fulfiller();

        // Remove the bound parent_node->child_node
        {
            let parent_node = self
                .nodes
                .get_mut(&parent_id)
                .expect("invalid parent node id");
            parent_node.requirements.remove(&requirement_id);
        }

        // Remove the bound child_node->parent_node, and tests if it was the last bound of child_node.
        let last_dependent = {
            let child_node = self
                .nodes
                .get_mut(&child_id)
                .expect("invalid child node id");
            child_node.dependents.remove(&requirement_id);
            child_node.dependents.is_empty()
        };

        // if it was the last bound of child_node, remove child_node
        if last_dependent {
            // Remove the child node's bounds, recursively
            let requirements = &self
                .nodes
                .get(&child_id)
                .expect("invalid child node id")
                .requirements
                .clone();
            for requirement_id in requirements {
                let requirement = self.requirements[&requirement_id].kind().clone();
                self.remove_requirement(child_id, &requirement)?;
            }

            // Remove the child node
            self.nodes.remove(&child_id);
        }

        // Remove the requirement
        self.requirements.remove(&requirement_id);

        Ok(())
    }
}

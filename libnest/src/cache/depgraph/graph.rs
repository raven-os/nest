use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::Write;
use std::marker::PhantomData;
use std::path::Path;

use failure::{format_err, Error, ResultExt};
use serde_derive::{Deserialize, Serialize};
use serde_json;

use crate::cache::available::{AvailablePackagesCacheQueryStrategy, QueryResult};
use crate::config::Config;
use crate::lock_file::LockFileOwnership;
use crate::package::{PackageFullName, PackageRequirement};

use super::super::errors::DependencyGraphErrorKind;
use super::node::{GroupName, Node, NodeID, NodeKind, NodeName, ROOT_ID};
use super::requirement::{
    Requirement, RequirementID, RequirementKind, RequirementManagementMethod,
};

/// The unsolved dependency graph: a serializable collection of [`Node`]s,
/// linked together with [`Requirement`]s.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct DependencyGraph<'lock_file> {
    next_node_id: NodeID,
    next_requirement_id: RequirementID,
    nodes: HashMap<NodeID, Node>,
    requirements: HashMap<RequirementID, Requirement>,
    node_names: HashMap<NodeName, NodeID>,
    #[serde(skip)]
    phantom: PhantomData<&'lock_file LockFileOwnership>,
}

impl<'lock_file> DependencyGraph<'lock_file> {
    pub(crate) fn new(
        phantom: PhantomData<&'lock_file LockFileOwnership>,
    ) -> DependencyGraph<'lock_file> {
        let mut nodes = HashMap::new();
        let mut node_names = HashMap::new();

        nodes.insert(
            ROOT_ID,
            Node::from(NodeKind::Group {
                name: GroupName::root_group(),
            }),
        );

        node_names.insert(NodeName::Group(GroupName::root_group()), ROOT_ID);

        DependencyGraph {
            next_node_id: ROOT_ID + 1,
            next_requirement_id: 0,
            nodes,
            requirements: HashMap::new(),
            node_names,
            phantom,
        }
    }

    #[inline]
    pub(crate) fn load_from_cache<P: AsRef<Path>>(
        path: P,
        phantom: PhantomData<&'lock_file LockFileOwnership>,
    ) -> Result<DependencyGraph<'lock_file>, Error> {
        let path = path.as_ref();

        if path.exists() {
            let file = File::open(path).with_context(|_| path.display().to_string())?;
            let graph =
                serde_json::from_reader(&file).with_context(|_| path.display().to_string())?;
            Ok(graph)
        } else {
            Ok(DependencyGraph::new(phantom))
        }
    }

    /// Saves the dependency graph back to the cache
    #[inline]
    pub fn save_to_cache<P: AsRef<Path>>(
        &self,
        path: P,
        _: &LockFileOwnership,
    ) -> Result<(), Error> {
        let path = path.as_ref();

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|_| parent.display().to_string())?;
        }

        let mut file = File::create(path).with_context(|_| path.display().to_string())?;
        serde_json::to_writer_pretty(&file, self).with_context(|_| path.display().to_string())?;
        writeln!(file)?;
        Ok(())
    }

    /// Returns the ID of the root of the graph
    #[inline]
    pub fn root_id(&self) -> NodeID {
        ROOT_ID
    }

    /// Consumes and returns the next node id
    #[inline]
    fn next_node_id(&mut self) -> NodeID {
        let id = self.next_node_id;
        self.next_node_id += 1;
        id
    }

    /// Consumes and returns the next requirement id
    #[inline]
    fn next_requirement_id(&mut self) -> RequirementID {
        let id = self.next_requirement_id;
        self.next_requirement_id += 1;
        id
    }

    /// Returns a reference over the internal [`HashMap`]<[`NodeID`], [`Node`]>.
    #[inline]
    pub fn nodes(&self) -> &HashMap<NodeID, Node> {
        &self.nodes
    }

    /// Returns a reference over the internal [`HashMap`]<[`RequirementID`], [`Requirement`]>.
    #[inline]
    pub fn requirements(&self) -> &HashMap<RequirementID, Requirement> {
        &self.requirements
    }

    /// Returns an iterator over the names of the groups in the graph.
    #[inline]
    pub fn groups(&self) -> impl Iterator<Item = &GroupName> {
        self.node_names.keys().filter_map(NodeName::group_name)
    }

    /// Returns an iterator over the names of the packages in the graph.
    #[inline]
    pub fn packages(&self) -> impl Iterator<Item = &PackageFullName> {
        self.node_names.keys().filter_map(NodeName::package_name)
    }

    /// Returns a reference over the internal [`HashMap`]<[`NodeName`], [`NodeID`]>.
    #[inline]
    pub fn node_names(&self) -> &HashMap<NodeName, NodeID> {
        &self.node_names
    }

    /// Returns the [`NodeID`] of a given package
    /// If no such ID is found, a [`DependencyGraphError`] is returned
    pub fn get_package_node_id(&self, name: &PackageFullName) -> Result<NodeID, Error> {
        let node_name = NodeName::Package(name.clone());

        self.node_names.get(&node_name).cloned().ok_or_else(|| {
            format_err!("{}", name)
                .context(DependencyGraphErrorKind::UnknownPackage)
                .into()
        })
    }

    /// Returns a reference to the [`Node`] of a given package
    /// If no such node is found, a [`DependencyGraphError`] is returned
    pub fn get_package_node(&self, name: &PackageFullName) -> Result<&Node, Error> {
        Ok(&self.nodes[&self.get_package_node_id(name)?])
    }

    /// Returns a mutable reference to the [`Node`] of a given package
    /// If no such node is found, a [`DependencyGraphError`] is returned
    pub fn get_package_node_mut(&mut self, name: &PackageFullName) -> Result<&mut Node, Error> {
        Ok(self
            .nodes
            .get_mut(&self.get_package_node_id(name)?)
            .expect("Invalid node id"))
    }

    /// Adds a given requirement as a dependency for a given node
    pub fn node_add_requirement(
        &mut self,
        node_id: NodeID,
        child_kind: RequirementKind,
        management_method: RequirementManagementMethod,
    ) -> RequirementID {
        // Create the requirement and insert it.
        let requirement_id = self.next_requirement_id();
        self.requirements.insert(
            requirement_id,
            Requirement::from(child_kind, management_method, node_id),
        );

        // Mark the requirement as a dependency of the node
        self.nodes
            .get_mut(&node_id)
            .expect("invalid parent node id")
            .requirements_mut()
            .insert(requirement_id);
        requirement_id
    }

    /// Tests by value if a group has a specific requirement
    pub fn node_has_requirement(&self, node: &Node, value: &RequirementKind) -> bool {
        for requirement_id in node.requirements() {
            let node_requirement = self
                .requirements
                .get(&requirement_id)
                .expect("invalid requirement id");

            if node_requirement.kind() == value {
                return true;
            }
        }
        false
    }

    /// Removes all requirements of `node_id` with the kind `requirement_kind`.
    pub fn node_remove_requirement(&mut self, node_id: NodeID, requirement_kind: RequirementKind) {
        // Collect all requirements IDs whose kind match the given one
        let requirement_ids = self
            .nodes
            .get(&node_id)
            .expect("invalid parent node id")
            .requirements()
            .iter()
            .filter(|requirement_id| {
                let kind = self
                    .requirements
                    .get(requirement_id)
                    .expect("invalid requirement id")
                    .kind();

                *kind == requirement_kind
            })
            .cloned()
            .collect::<Vec<_>>();

        for requirement_id in requirement_ids {
            if let Some(requirement) = self.requirements.get(&requirement_id) {
                // Remove requirement from dependent/fulfilled node
                self.nodes
                    .get_mut(&requirement.fulfilled_node_id())
                    .expect("invalid node id")
                    .requirements_mut()
                    .remove(&requirement_id);

                // Remove requirement from dependency/fulfilling node
                if let Some(child_id) = requirement.fulfilling_node_id() {
                    self.nodes
                        .get_mut(&child_id)
                        .expect("invalid node id")
                        .dependents_mut()
                        .remove(&requirement_id);
                }
            }

            // Remove requirement from requirement table
            self.requirements.remove(&requirement_id);
        }
    }

    /// Fulfills a requirement using a given node
    fn node_fulfill_requirement(&mut self, target: NodeID, req: RequirementID) {
        // Mark the requirement as fulfilled by the node
        *self
            .requirements
            .get_mut(&req)
            .expect("invalid requirement id")
            .fulfilling_node_id_mut() = Some(target);

        // Mark the node as fulfilling (aka "depended on by") the requirement
        self.nodes
            .get_mut(&target)
            .expect("invalid node id")
            .dependents_mut()
            .insert(req);
    }

    /// Removes a requirement of the dependency graph
    /// The requirement is removed from both the dependency and the dependent points of view.
    pub fn remove_requirement(&mut self, requirement_id: RequirementID) {
        let requirement = &self.requirements[&requirement_id];

        // Remove the requirement from the dependent node
        let parent_node = self
            .nodes
            .get_mut(&requirement.fulfilled_node_id())
            .expect("invalid node id");

        parent_node.requirements_mut().remove(&requirement_id);

        // Remove the requirement from the dependency node, if it exists
        if let Some(child_id) = requirement.fulfilling_node_id() {
            let child_node = self.nodes.get_mut(&child_id).expect("invalid node id");
            child_node.dependents_mut().remove(&requirement_id);
        }

        // Remove the requirement from the requirement table.
        self.requirements.remove(&requirement_id);
    }

    /// Creates a new node with the given package
    pub fn add_package_node(&mut self, package: QueryResult) -> Result<NodeID, Error> {
        let node_name = NodeName::Package(package.full_name());

        if self.node_names.contains_key(&node_name) {
            Err(format_err!("{}", &node_name)
                .context(DependencyGraphErrorKind::PackageAlreadyExists)
                .into())
        } else {
            let node_id = self.next_node_id();

            self.nodes
                .insert(node_id, Node::from(NodeKind::Package { id: package.id() }));

            for dependency in package.manifest().dependencies() {
                let kind = RequirementKind::Package {
                    package_req: dependency.clone(),
                };
                self.node_add_requirement(node_id, kind, RequirementManagementMethod::Auto);
            }

            self.node_names.insert(node_name, node_id);
            Ok(node_id)
        }
    }

    /// Creates a new node, which is a group of the given name
    pub fn add_group_node(&mut self, name: GroupName) -> Result<NodeID, Error> {
        let node_name = NodeName::Group(name.clone());

        if self.node_names.contains_key(&node_name) {
            Err(format_err!("{}", &name.as_str())
                .context(DependencyGraphErrorKind::GroupAlreadyExists)
                .into())
        } else {
            let group_id = self.next_node_id();
            let group = Node::from(NodeKind::Group { name });

            // Insert the group in the node names table
            self.node_names.insert(node_name, group_id);

            // Insert the group node in the nodes table
            self.nodes.insert(group_id, group);

            Ok(group_id)
        }
    }

    /// Removes a node from the dependency graph, and all requirements linked from/to it
    pub fn remove_node(&mut self, node_id: NodeID) {
        let dependents = self
            .nodes
            .get(&node_id)
            .expect("invalid node id")
            .dependents()
            .clone();

        let requirements = self
            .nodes
            .get(&node_id)
            .expect("invalid node id")
            .requirements()
            .clone();

        // Remove requirements held/fulfilled by this node
        for requirement_id in dependents {
            let parent_id = self
                .requirements
                .get_mut(&requirement_id)
                .expect("invalid requirement id")
                .fulfilled_node_id();

            // Remove the requirement from the parent node.
            // No need to remove it from the current node as it will be deleted anyway.
            self.nodes
                .get_mut(&parent_id)
                .expect("invalid node id")
                .requirements_mut()
                .remove(&requirement_id);

            // Remove the requirement from the global requirements table
            self.requirements.remove(&requirement_id);
        }

        // Remove requirements needed by this node
        for requirement_id in requirements {
            let child_id_opt = self
                .requirements
                .get_mut(&requirement_id)
                .expect("invalid requirement id")
                .fulfilling_node_id();

            if let Some(child_id) = child_id_opt {
                // Remove the requirement from the child node.
                // No need to remove it from the current node as it will be deleted anyway.
                self.nodes
                    .get_mut(&child_id)
                    .expect("invalid node id")
                    .dependents_mut()
                    .remove(&requirement_id);
            }

            // Remove the requirement from the global requirements table
            self.requirements.remove(&requirement_id);
        }

        // Remove the node from the node table and the groups/packages tables
        match self.nodes[&node_id].kind() {
            NodeKind::Group { name } => {
                self.node_names.remove(&NodeName::Group(name.clone()));
            }
            NodeKind::Package { id } => {
                self.node_names
                    .remove(&NodeName::Package(id.clone().into()));
            }
        }

        // Remove the node from the nodes table
        self.nodes.remove(&node_id);
    }

    fn remove_orphans_rec(&self, marks: &mut HashSet<NodeID>, node_id: NodeID) {
        if !marks.contains(&node_id) {
            marks.insert(node_id);

            let node = &self.nodes[&node_id];
            for requirement_id in node.requirements() {
                let requirement = &self.requirements[&requirement_id];
                if let Some(node_id) = requirement.fulfilling_node_id() {
                    self.remove_orphans_rec(marks, *node_id);
                }
            }
        }
    }

    /// Removes orphan nodes from the dependency graph, that is, nodes not fulfilling any requirement
    fn remove_orphan_nodes(&mut self) {
        let mut to_keep = HashSet::new();

        self.remove_orphans_rec(&mut to_keep, ROOT_ID);

        let to_remove: Vec<_> = self
            .nodes
            .keys()
            .filter(|node_id| !to_keep.contains(node_id))
            .cloned()
            .collect();

        to_remove
            .into_iter()
            .for_each(|node_id| self.remove_node(node_id));
    }

    fn find_node_for_requirement(&self, requirement: &PackageRequirement) -> Option<NodeID> {
        self.node_names
            .iter()
            .find(|(node_name, _)| {
                if let Some(full_name) = node_name.package_name() {
                    requirement.name() == full_name.name()
                        && requirement.category() == full_name.category()
                } else {
                    false
                }
            })
            .map(|(_, id)| *id)
    }

    fn solve_package_requirement(
        &mut self,
        config: &Config,
        requirement: PackageRequirement,
    ) -> Result<NodeID, Error> {
        // The list of requirements the package must fulfill.
        let mut requirements = Vec::new();
        let node_id_opt = self.find_node_for_requirement(&requirement);

        // Test whether a package with the same PackageFullName is already within the dependency graph
        if let Some(package_node_id) = node_id_opt {
            let node = &self.nodes[&package_node_id];

            // Since a version of the package is already in the graph, test whether it matches the new requirement
            if let NodeKind::Package { id } = node.kind() {
                if requirement.matches(id) {
                    // If that's the case, we can stop here, as the requirement is already fulfilled
                    return Ok(package_node_id);
                }
            }

            // At this point, a version of the package is already in the graph, but it does not match the new requirement.
            // Therefore, the version of the package is going to change in order to solve the new requirement.
            //
            // However, the old requirements on the installed version of the package should be preserved,
            // thus we add them to the requirements to fulfill.

            let requirement_kinds = node
                .dependents()
                .iter()
                .map(|requirement_id| &self.requirements[requirement_id])
                .map(|requirement| requirement.kind());

            for requirement_kind in requirement_kinds {
                if let RequirementKind::Package { package_req } = requirement_kind {
                    requirements.push(package_req.clone());
                }
            }
        }

        // We add the new requirement to the requirements to fulfill
        requirements.push(requirement.clone());

        // Look for the newest version matching all the requirements
        let find_matching_packages = || -> Result<Option<QueryResult>, Error> {
            let available_packages = config
                .available_packages_cache_internal(self.phantom)
                .query(&requirement.clone().any_version().into())
                .set_strategy(AvailablePackagesCacheQueryStrategy::AllMatchesSorted)
                .perform();

            for package in available_packages? {
                let is_valid = requirements
                    .iter()
                    .all(|requirement| requirement.matches(&package.id()));
                if is_valid {
                    return Ok(Some(package));
                }
            }
            Ok(None)
        };

        let package = find_matching_packages()?.ok_or_else(|| {
            format_err!("{}", requirement)
                .context(DependencyGraphErrorKind::RequirementSolvingError)
        })?;

        // If the new version is different from the old one, remove the old one
        if let Some(node_id) = node_id_opt {
            let node = self.nodes.get_mut(&node_id).expect("invalid node id");
            let id = package.id();

            if (*node.kind() != NodeKind::Package { id: id.clone() }) {
                *node.kind_mut() = NodeKind::Package { id };
                node.requirements_mut().clear();
                self.solve_node(config, node_id)?;
                Ok(node_id)
            } else {
                Ok(node_id)
            }
        } else {
            let node_id = self.add_package_node(package)?;
            self.solve_node(config, node_id)?;
            Ok(node_id)
        }
    }

    /// Solves the requirement with the given ID
    pub fn solve_requirement(
        &mut self,
        config: &Config,
        requirement_id: RequirementID,
    ) -> Result<(), Error> {
        // Avoid borrowing requirement for too long by pre-computing the interesting values.
        let (unsolved, kind) = {
            let requirement = &self.requirements[&requirement_id];
            (
                requirement.fulfilling_node_id().is_none(),
                requirement.kind().clone(),
            )
        };

        // The requirement only has to be solved if it is unsolved
        if unsolved {
            let solver_id = match &kind {
                RequirementKind::Package { package_req } => {
                    self.solve_package_requirement(config, package_req.clone())?
                }
                RequirementKind::Group { name } => {
                    let group_id = self.node_names.get(&name.clone().into()).ok_or_else(|| {
                        format_err!("{}", name.as_str())
                            .context(DependencyGraphErrorKind::GroupNotFound)
                    })?;
                    *group_id
                }
            };

            // Update the requirement
            self.node_fulfill_requirement(solver_id, requirement_id)
        }
        Ok(())
    }

    fn solve_node(&mut self, config: &Config, node_id: NodeID) -> Result<(), Error> {
        let requirements = self.nodes[&node_id].requirements().clone();

        // Solve all requirements
        for requirement_id in &requirements {
            self.solve_requirement(config, *requirement_id)?;
        }

        // Repeat for each requirement's fulfilling node
        for requirement_id in &requirements {
            let node_id = self.requirements[&requirement_id]
                .fulfilling_node_id()
                .expect("expected a fulfilling node after solving the dependent node");
            self.solve_node(config, node_id)?;
        }
        Ok(())
    }

    /// Solves the graph (attempts to fulfill every requirement)
    pub fn solve(&mut self, config: &Config) -> Result<(), Error> {
        self.solve_node(config, ROOT_ID)?;
        self.remove_orphan_nodes();
        Ok(())
    }

    /// Updates the graph by removing automatic requirements, and solving again
    pub fn update(&mut self, config: &Config) -> Result<(), Error> {
        // First, remove auto requirements. Static requirements against packages are set as unsolved.
        let mut marks = HashSet::new();
        for (requirement_id, requirement) in &mut self.requirements {
            match requirement.management_method() {
                RequirementManagementMethod::Auto => {
                    marks.insert(*requirement_id);
                }
                RequirementManagementMethod::Static => {
                    // Unsolve it
                    if let Some(child_id) = requirement.fulfilling_node_id() {
                        let child = self.nodes.get_mut(&child_id).expect("invalid node id");
                        child.dependents_mut().remove(requirement_id);
                    }
                    *requirement.fulfilling_node_id_mut() = None;
                }
            }
        }

        for requirement_id in marks {
            self.remove_requirement(requirement_id);
        }

        // Then, remove orphan nodes
        // We should only have groups left, roughly.
        self.remove_orphan_nodes();

        // Solve the graph
        self.solve(config)
    }
}

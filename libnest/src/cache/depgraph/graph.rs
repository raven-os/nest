use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use failure::{Error, ResultExt};
use serde_derive::{Deserialize, Serialize};
use serde_json;

use crate::package::PackageFullName;

use super::node::{GroupName, Node, NodeID, NodeKind, ROOT_ID};
use super::requirement::{Requirement, RequirementID};

/// The unsolved dependency graph: a serializable collection of [`Node`]s,
/// linked together with [`Requirement`]s.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct DependencyGraph {
    next_node_id: NodeID,
    next_requirement_id: RequirementID,
    nodes: HashMap<NodeID, Node>,
    requirements: HashMap<RequirementID, Requirement>,
    packages: HashMap<PackageFullName, NodeID>,
    groups: HashMap<GroupName, NodeID>,
}

impl DependencyGraph {
    #[allow(dead_code)] // TODO: Remove this when the function is used
    pub(crate) fn new() -> DependencyGraph {
        let mut nodes = HashMap::new();
        let mut groups = HashMap::new();

        nodes.insert(
            ROOT_ID,
            Node::from(NodeKind::Group {
                name: GroupName::root_group(),
            }),
        );

        groups.insert(GroupName::root_group(), ROOT_ID);

        DependencyGraph {
            next_node_id: ROOT_ID + 1,
            next_requirement_id: 0,
            nodes,
            requirements: HashMap::new(),
            groups,
            packages: HashMap::new(),
        }
    }

    #[allow(dead_code)] // TODO: Remove this when the function is used
    #[inline]
    pub(crate) fn load_from_cache<P: AsRef<Path>>(path: P) -> Result<DependencyGraph, Error> {
        let path = path.as_ref();

        if path.exists() {
            let file = File::open(path).with_context(|_| path.display().to_string())?;
            let graph =
                serde_json::from_reader(&file).with_context(|_| path.display().to_string())?;
            Ok(graph)
        } else {
            Ok(DependencyGraph::new())
        }
    }

    #[allow(dead_code)] // TODO: Remove this when the function is used
    #[inline]
    pub fn save_to_cache<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let path = path.as_ref();

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|_| parent.display().to_string())?;
        }

        let mut file = File::create(path).with_context(|_| path.display().to_string())?;
        serde_json::to_writer_pretty(&file, self).with_context(|_| path.display().to_string())?;
        writeln!(file)?;
        Ok(())
    }
}

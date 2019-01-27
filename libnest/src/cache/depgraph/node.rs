use std::collections::HashSet;
use std::str::FromStr;

use failure::Error;
use lazy_static::lazy_static;
use regex::Regex;
use serde_derive::{Deserialize, Serialize};

use crate::package::PackageID;

use super::super::errors::{GroupNameError, GroupNameErrorKind};
use super::RequirementID;

/// Type representing unique identifiers of a node in the dependency graph
pub type NodeID = usize;

/// Type representing the name of a group.
/// Its value must match the regular expression "@[a-z0-9]+"
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct GroupName(String);

pub(crate) static ROOT_ID: NodeID = 0;
pub(crate) static ROOT_NAME: &str = "@root";

impl GroupName {
    /// Returns the [`GroupName`] of the root group
    pub fn root_group() -> GroupName {
        GroupName::from_str(ROOT_NAME).unwrap()
    }
}

impl std::ops::Deref for GroupName {
    type Target = String;

    #[inline]
    fn deref(&self) -> &String {
        &self.0
    }
}

lazy_static! {
    static ref REGEX_GROUP_NAME: Regex = Regex::new(r"^@[a-z0-9]$").unwrap();
}

impl FromStr for GroupName {
    type Err = Error;

    fn from_str(str_repr: &str) -> Result<Self, Self::Err> {
        if REGEX_GROUP_NAME.is_match(str_repr) {
            Ok(GroupName(str_repr.to_string()))
        } else {
            Err(GroupNameError::from(GroupNameErrorKind::InvalidGroupName).into())
        }
    }
}

/// The kind of a node in the dependency graph
///
/// A node is not necessarily a single package, it can also be a group: a named list of requirements.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum NodeKind {
    /// The node is a group
    Group {
        /// The name of the group
        name: GroupName,
    },
    /// The node is a package
    Package {
        /// The [`PackageID`] of this node.
        id: PackageID,
    },
}

/// A node of the dependency graph.
///
/// A node is represented by a [`NodeKind`][1], a set of [`NodeRequirement`][2] that must
/// be satisfied for the graph to be valid, and a set of requirements that other nodes have on this one.
///
/// [1]: enum.NodeKind.html
/// [2]: requirement/struct.NodeRequirement.html
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct Node {
    kind: NodeKind,
    requirements: HashSet<RequirementID>,
    dependents: HashSet<RequirementID>,
}

impl Node {
    #[inline]
    pub fn from(kind: NodeKind) -> Node {
        Node {
            kind,
            requirements: HashSet::new(),
            dependents: HashSet::new(),
        }
    }

    #[allow(dead_code)] // TODO: Remove this when the function is used
    #[inline]
    pub fn requirements(&self) -> &HashSet<RequirementID> {
        &self.requirements
    }

    #[allow(dead_code)] // TODO: Remove this when the function is used
    #[inline]
    pub fn requirements_mut(&mut self) -> &mut HashSet<RequirementID> {
        &mut self.requirements
    }

    #[allow(dead_code)] // TODO: Remove this when the function is used
    #[inline]
    pub fn dependents(&self) -> &HashSet<RequirementID> {
        &self.dependents
    }

    #[allow(dead_code)] // TODO: Remove this when the function is used
    #[inline]
    pub fn dependents_mut(&mut self) -> &mut HashSet<RequirementID> {
        &mut self.dependents
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.kind {
            NodeKind::Group { name, .. } => write!(f, "@{}", name.as_str()),
            NodeKind::Package { id, .. } => write!(f, "{}", id),
        }
    }
}

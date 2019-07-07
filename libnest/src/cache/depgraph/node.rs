use std::collections::HashSet;
use std::str::FromStr;

use failure::Error;
use lazy_static::lazy_static;
use regex::Regex;
use serde_derive::{Deserialize, Serialize};

use crate::package::{PackageFullName, PackageID};

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
    static ref REGEX_GROUP_NAME: Regex = Regex::new(r"^@[a-z0-9]+$").unwrap();
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
    /// Creates a node from a given kind, with no dependents and no requirements
    #[inline]
    pub fn from(kind: NodeKind) -> Node {
        Node {
            kind,
            requirements: HashSet::new(),
            dependents: HashSet::new(),
        }
    }

    /// Returns a reference to the kind of this node
    #[allow(dead_code)] // TODO: Remove this when the function is used
    #[inline]
    pub fn kind(&self) -> &NodeKind {
        &self.kind
    }

    /// Returns a mutable reference to the kind of this node
    #[inline]
    pub fn kind_mut(&mut self) -> &mut NodeKind {
        &mut self.kind
    }

    /// Returns a reference to the set of [`RequirementID`]s needed by this node
    #[allow(dead_code)] // TODO: Remove this when the function is used
    #[inline]
    pub fn requirements(&self) -> &HashSet<RequirementID> {
        &self.requirements
    }

    /// Returns a mutable reference to the set of [`RequirementID`]s needed by this node
    #[allow(dead_code)] // TODO: Remove this when the function is used
    #[inline]
    pub fn requirements_mut(&mut self) -> &mut HashSet<RequirementID> {
        &mut self.requirements
    }

    /// Returns a reference to the set of [`RequirementID`]s held on this node
    #[allow(dead_code)] // TODO: Remove this when the function is used
    #[inline]
    pub fn dependents(&self) -> &HashSet<RequirementID> {
        &self.dependents
    }

    /// Returns a mutable reference to the set of [`RequirementID`]s held on this node
    #[allow(dead_code)] // TODO: Remove this when the function is used
    #[inline]
    pub fn dependents_mut(&mut self) -> &mut HashSet<RequirementID> {
        &mut self.dependents
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.kind {
            NodeKind::Group { name, .. } => write!(f, "{}", name.as_str()),
            NodeKind::Package { id, .. } => write!(f, "{}", id),
        }
    }
}

/// The name of a node (that is, the [`GroupName`] or the [`PackageFullName`] for this node)
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum NodeName {
    /// The node name describes a group
    Group(GroupName),

    /// The node name describes a package
    Package(PackageFullName),
}

impl NodeName {
    /// Retrieves the [`GroupName`] if the node name describes a group
    pub fn group_name(&self) -> Option<&GroupName> {
        if let NodeName::Group(group_name) = self {
            Some(group_name)
        } else {
            None
        }
    }

    /// Retrieves the [`PackageFullName`] if the node name describes a package
    pub fn package_name(&self) -> Option<&PackageFullName> {
        if let NodeName::Package(full_name) = self {
            Some(full_name)
        } else {
            None
        }
    }
}

impl std::fmt::Display for NodeName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            NodeName::Group(group_name) => f.write_str(group_name.as_str()),
            NodeName::Package(full_name) => f.write_fmt(format_args!("{}", full_name)),
        }
    }
}

impl From<NodeKind> for NodeName {
    fn from(kind: NodeKind) -> Self {
        match kind {
            NodeKind::Group { name } => NodeName::Group(name),
            NodeKind::Package { id } => NodeName::Package(id.into()),
        }
    }
}

impl From<GroupName> for NodeName {
    fn from(group_name: GroupName) -> Self {
        NodeName::Group(group_name)
    }
}

impl From<PackageFullName> for NodeName {
    fn from(full_name: PackageFullName) -> Self {
        NodeName::Package(full_name)
    }
}

impl serde::Serialize for NodeName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        match self {
            NodeName::Group(name) => serializer.serialize_str(name),
            NodeName::Package(full_name) => full_name.serialize(serializer),
        }
    }
}

struct NodeNameDeserializeVisitor;

impl<'de> serde::de::Visitor<'de> for NodeNameDeserializeVisitor {
    type Value = NodeName;

    #[inline]
    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str("a node name")
    }

    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value.chars().next() {
            Some('@') => GroupName::from_str(value).map(NodeName::Group).map_err(|_| {
                E::custom(
                    "the group's name doesn't follow the convention `@name`",
                )
            }),
            _ => PackageFullName::from_str(value).map(NodeName::Package).map_err(|_| {
                E::custom(
                    "the package's full name doesn't follow the convention `repository::category/name`",
                )
            }),
        }
    }
}

impl<'a> serde::Deserialize<'a> for NodeName {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        deserializer.deserialize_str(NodeNameDeserializeVisitor)
    }
}

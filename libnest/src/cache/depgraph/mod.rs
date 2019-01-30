//! Module to manipulate the dependency graph

mod diff;
mod graph;
mod node;
mod requirement;
pub use self::diff::DependencyGraphDiff;
pub use self::graph::DependencyGraph;
pub use self::node::{GroupName, NodeID, NodeKind};
pub use self::requirement::{Requirement, RequirementID, RequirementKind};

/*
    Appellation: node <module>
    Contrib: @FL03
*/
//! This module implements the base node structure used throughout the system.
//!
//! Abstractly speaking, every node is designed around the structure of a
pub use self::node::NodeBase;

mod node;

/// A node is a point within c-space
pub trait Node {
    type Ctx;
    type Rel; // The `relative` c-space for the node

    /// The address of the node within c-space; often a unique identifier
    fn position(&self) -> &str;
}

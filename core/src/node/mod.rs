/*
    Appellation: node <module>
    Contrib: @FL03
*/
//! This module implements the base node structure used throughout the system.
//!
//! Abstractly speaking, every node is designed around the structure of a
pub use self::node::NodeBase;

mod node;

/// The base trait for all nodes considered within the system;
///
pub trait RawNode {
    type Ctx;

    seal!();
}

pub trait PositionedNode: RawNode {
    #[doc(hidden)]
    /// return a reference to the node's position within space
    fn position(&self) -> &str;
}

/// A [RelativeNode] extends the [RawNode] trait by adding a `relative` c-space to the node,
/// tethering the object in a manner akin to the neo-riemannian theory. Logically, the
/// `relative` of a `relative` should be the same as the original. This follows as LPR
/// transformations are invertivble, and thus consecutive applications of the same
/// transformation onto the object should result in the original object.
pub trait RelativeNode: RawNode {
    type Rel: RelativeNode<Rel = Self>; // the `relative` of a `relative` should be the same as the original; same as in the neo-riemannian theory
}

impl<T> RawNode for Vec<T> {
    type Ctx = T;
}

impl<T> RawNode for Box<dyn RawNode<Ctx = T>> {
    type Ctx = T;
}

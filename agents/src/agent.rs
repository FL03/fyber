/*
    Appellation: agent <module>
    Contrib: @FL03
*/
//! An `agent` is defined as an abstract model of computation defining entities capable of
//! perceivivng sensory information and acting upon it.

use crate::components::ComponentRegistry;

/// An `agent` is defined as an abstract model of computation defining entities capable of
/// perceivivng sensory information and acting upon it.
/// 
/// Each agent is associated with a node in the system. The architecture of the system is
/// such that the agent is able to interact with the node and its components while being
/// just a single-step away from
pub struct Agent<N> {
    pub components: ComponentRegistry,
    pub node: N, // The node that the agent is associated with

}

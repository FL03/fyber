/*
    Appellation: agent <module>
    Contrib: @FL03
*/
use crate::components::ComponentRegistry;

/// An `agent` is defined as an abstract model of computation defining entities capable of
/// perceivivng sensory information and acting upon it.
///
/// Each agent is associated with a node in the system. The architecture of the system is
/// such that the agent is able to interact with the node and its components while being
/// just a single-step away from
pub struct Agent<N> {
    pub components: ComponentRegistry,
    pub name: String,
    pub node: N, // The (physical or virtual) node that the agent is associated with
    pub prev_state: String, // The previous state of the agent
}

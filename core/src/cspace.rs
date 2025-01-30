/*
    Appellation: cspace <module>
    Contrib: @FL03
*/

/// [`CSpace`] is a trait defining the abstract concept of a configuration space. A c-space is
/// a space representing all possible configurations of a given system. These spaces are most
/// commonly associated with robotics and motion planning, where the c-space represents every
/// possible position, orientation, and configuration of a robot within an environment.
///
/// Here, we use the concept of `c-space` to uniformly describe all possible "states" of an
/// node within our system architecture. This allows us to define a common interface for
/// interacting with nodes and their configurations, regardless of the specific implementation
/// of the node.
pub trait CSpace {
    fn check_path(&self, path: Vec<String>) -> bool;

    fn is_valid(&self) -> bool;
}

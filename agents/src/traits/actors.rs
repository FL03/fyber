/*
    Appellation: actors <module>
    Contrib: @FL03
*/
use fyber_core::{node::Node, FyberResult};

pub trait PhysicalNode: Node {
    fn physicalize(&self) -> Self::Rel;
}

pub trait VirtualNode: Node {
    fn virtualize(&self) -> Self::Rel;
}

pub trait NodeContext {
    type Config;
}
/// An [Actor] is any _actionable_ node that may possibly change the state of the system
pub trait Actor: Node {
    fn handle(&self, ctx: &Self::Ctx) -> FyberResult<()>;
}

pub trait Observer: Node {
    fn observe(&self, ctx: &Self::Ctx) -> FyberResult<()>;
}

/*
    Appellation: actors <module>
    Contrib: @FL03
*/
use fyber_core::{FyberResult, node::RawNode};

pub trait RawContext {
    type Config;
}

pub trait RawContainer {
    type Data;
}

pub trait State {
    type Ctx: RawContext;
    type Data: RawContainer;
}

/// An [Actor] is any _actionable_ node that may possibly change the state of the system
pub trait Actor {
    type Node: RawNode;

    fn handle<Ctx>(&self, ctx: &Ctx) -> FyberResult<()>
    where
        Ctx: RawContext,
        Self::Node: RawNode<Ctx = Ctx>;
}

pub trait Observer {
    type Node: RawNode;

    fn observe<Ctx>(&self, ctx: &Ctx) -> FyberResult<()>
    where
        Ctx: RawContext,
        Self::Node: RawNode<Ctx = Ctx>;
}

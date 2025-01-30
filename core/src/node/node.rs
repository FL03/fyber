/*
    Appellation: node <module>
    Contrib: @FL03
*/
use super::RawNode;
use core::marker::PhantomData;

pub struct NodeBase<N: RawNode> {
    data: N,
    _type: PhantomData<N::Ctx>,
}

impl<N: RawNode> NodeBase<N> {
    pub fn new(data: N) -> Self {
        Self {
            data,
            _type: PhantomData,
        }
    }
}

/*
    Appellation: actor <module>
    Contrib: @FL03
*/

#[doc(hidden)]
/// Every component capable of being executed by an actor
pub trait ComponentModel {}

// type DynamicComponent = Box<dyn Component>;


impl ComponentModel for wasmtime::component::Component {}

pub trait Node {
    type Cmp: ComponentModel; // Typically, as wasmtime::Component
    type Ctx;
    type Data;
    
    fn components(&self) -> Vec<Self::Cmp>;
}
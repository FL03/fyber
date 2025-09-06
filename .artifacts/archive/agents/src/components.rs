/*
    Appellation: actor <module>
    Contrib: @FL03
*/
use std::collections::HashMap;

#[doc(hidden)]
/// Every component capable of being executed by an actor
pub trait ComponentModel {}

impl ComponentModel for wasmtime::component::Component {}

pub struct ComponentRegistry {
    components: HashMap<String, Box<dyn ComponentModel>>,
}

impl ComponentRegistry {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, component: Box<dyn ComponentModel>) {
        self.components.insert(key, component);
    }

    pub fn get(&self, key: &str) -> Option<&Box<dyn ComponentModel>> {
        self.components.get(key)
    }

    pub fn remove(&mut self, key: &str) -> Option<Box<dyn ComponentModel>> {
        self.components.remove(key)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.components.contains_key(key)
    }

    pub fn len(&self) -> usize {
        self.components.len()
    }
}

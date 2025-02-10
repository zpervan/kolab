use super::Component;
use std::collections::HashMap;
use uuid::Uuid;

pub struct CircuitStore {
    components: HashMap<Uuid, Box<dyn Component>>
}

impl CircuitStore {
    pub fn new() -> Self {
        Self {
            components: HashMap::new()
        }
    }

    pub fn upsert(&mut self, component: Box<dyn Component>) {
        self.components.insert(component.id(), component);
    }
}
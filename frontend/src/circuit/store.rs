use super::Component;
use serde_json::de::IoRead;
use std::collections::HashMap;
use uuid::Uuid;

pub struct CircuitStore {
    components: HashMap<Uuid, Box<dyn Component>>,
    pending_component_id: Option<Uuid>,
}

impl CircuitStore {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            pending_component_id: None,
        }
    }

    pub fn upsert(&mut self, component: Box<dyn Component>) {
        self.components.insert(component.id(), component);
    }

    pub fn get(&self, id: Uuid) -> Option<&Box<dyn Component>> {
        self.components.get(&id)
    }

    pub fn get_mut(&mut self, id: &Uuid) -> Option<&mut Box<dyn Component>> {
        self.components.get_mut(id)
    }

    pub fn components(&self) -> Vec<&Box<dyn Component>> {
        self.components.iter().map(|(_, c)| c).collect()
    }

    pub fn set_pending_component_id(&mut self, id: Uuid) {
        self.pending_component_id = Some(id);
    }
    
    pub fn pending_component_id(&self) -> Option<Uuid> {
        self.pending_component_id
    }
}

use super::Component;
use std::collections::HashMap;
use uuid::Uuid;

pub struct CircuitStore {
    components: HashMap<Uuid, Box<dyn Component>>,
    pending_component: Option<Box<dyn Component>>,
}

impl CircuitStore {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            pending_component: None,
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
        self.components.values().collect()
    }

    pub fn set_pending_component(&mut self, component: Box<dyn Component>) {
        self.pending_component = Some(component);
    }

    pub fn clear_pending_component(&mut self) -> Option<Box<dyn Component>> {
        let pending_component = self.pending_component.take();
        self.pending_component = None;

        pending_component
    }

    pub fn pending_component(&self) -> Option<&dyn Component> {
        self.pending_component.as_deref()
    }

    pub fn pending_component_mut(&mut self) -> Option<&mut Box<dyn Component>> {
        self.pending_component.as_mut()
    }
}

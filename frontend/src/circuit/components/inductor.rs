use crate::circuit::components::{Component, ComponentType, CAPACITOR_COMPONENT_SIZE};
use eframe::emath::{Pos2, Rect};
use std::ops::Add;
use uuid::Uuid;

pub struct Inductor {
    id: Uuid,
    value: Option<f32>,
    position: Pos2,
}

impl Inductor {
    pub fn new(initial_position: Pos2) -> Self {
        Self {
            id: Uuid::new_v4(),
            value: None,
            position: initial_position,
        }
    }
}

impl Component for Inductor {
    fn id(&self) -> Uuid {
        self.id
    }

    fn component_type(&self) -> ComponentType {
        ComponentType::Inductor
    }

    fn set_value(&mut self, value: f32) {
        self.value = Some(value);
    }

    fn value(&self) -> Option<f32> {
        self.value
    }

    fn set_position(&mut self, pos: Pos2) {
        self.position = pos;
    }

    fn position(&self) -> Pos2 {
        self.position
    }

    fn bounds_component(&self) -> Rect {
        Rect {
            min: self.position,
            max: self.position.add(CAPACITOR_COMPONENT_SIZE),
        }
    }
}

use crate::circuit::components::DEFAULT_COMPONENT_SIZE;
use crate::circuit::Component;
use egui::{Pos2, Rect, Vec2};
use std::ops::Add;
use uuid::Uuid;

pub struct Resistor {
    id: Uuid,
    value: Option<f32>,
    position: Pos2,
}

impl Resistor {
    pub fn new(initial_position: Pos2) -> Self {
        Self {
            id: Uuid::new_v4(),
            value: None,
            position: initial_position,
        }
    }
}

impl Component for Resistor {
    fn id(&self) -> Uuid {
        self.id
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

    fn bounds(&self) -> Rect {
        Rect {
            min: self.position,
            max: self.position.add(DEFAULT_COMPONENT_SIZE),
        }
    }

    fn is_hit(&self, pos: Pos2) -> bool {
        self.bounds().contains(pos)
    }
}

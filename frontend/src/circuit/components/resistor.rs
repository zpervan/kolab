use crate::circuit::Component;
use egui::Vec2;
use uuid::Uuid;

pub struct Resistor {
    id: Uuid,
    value: Option<f32>,
    position: Option<Vec2>,
}

impl Resistor {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            value: None,
            position: None,
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

    fn set_position(&mut self, pos: Vec2) {
        self.position = Some(pos);
    }

    fn position(&self) -> Option<Vec2> {
        self.position
    }
}

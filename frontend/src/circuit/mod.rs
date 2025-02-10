pub mod store;

use egui::Vec2;
use uuid::Uuid;

pub trait Component {
    fn id(&self) -> Uuid;
    
    fn set_value(&mut self, value: f32);
    
    fn value(&self) -> Option<f32>;
    
    fn position(&self) -> Option<Vec2>;
}

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
    fn id(&self) -> Uuid { self.id }

    fn set_value(&mut self, value: f32) {
        self.value = Some(value);
    }

    fn value(&self) -> Option<f32> {
        self.value
    }
    
    fn position(&self) -> Option<Vec2> { self.position }
}

pub struct Capacitor {
    id: Uuid,
    value: Option<f32>,
    position: Option<Vec2>,
}

impl Capacitor {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            value: None,
            position: None,
        }
    }
}

impl Component for Capacitor {
    fn id(&self) -> Uuid { self.id }

    fn set_value(&mut self, value: f32) {
        self.value = Some(value);
    }

    fn value(&self) -> Option<f32> {
        self.value
    }

    fn position(&self) -> Option<Vec2> { self.position }
}

pub struct Inductor {
    id: Uuid,
    value: Option<f32>,
    position: Option<Vec2>,
}

impl Inductor {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            value: None,
            position: None,
        }
    }
}

impl Component for Inductor {
    fn id(&self) -> Uuid { self.id }
    
    fn set_value(&mut self, value: f32) {
        self.value = Some(value);
    }

    fn value(&self) -> Option<f32> {
        self.value
    }

    fn position(&self) -> Option<Vec2> { self.position }
}
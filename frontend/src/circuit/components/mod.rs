use egui::Vec2;

pub mod capacitor;
pub mod inductor;
pub mod resistor;

pub static RESISTOR_COMPONENT_SIZE: Vec2 = Vec2::new(100.0, 25.0);
pub static CAPACITOR_COMPONENT_SIZE: Vec2 = Vec2::new(50.0, 25.0);

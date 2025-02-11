pub mod actor;
pub mod components;
pub mod store;

use egui::Vec2;
use uuid::Uuid;

pub trait Component {
    fn id(&self) -> Uuid;

    fn set_value(&mut self, value: f32);

    fn value(&self) -> Option<f32>;

    fn set_position(&mut self, pos: Vec2);

    fn position(&self) -> Option<Vec2>;
}

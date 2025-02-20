pub mod actor;
pub mod components;
pub mod store;

use eframe::emath::Pos2;
use egui::{Rect, Vec2};
use uuid::Uuid;

pub trait Component {
    fn id(&self) -> Uuid;

    fn set_value(&mut self, value: f32);

    fn value(&self) -> Option<f32>;

    fn set_position(&mut self, pos: Pos2);

    fn position(&self) -> Pos2;

    fn bounds(&self) -> Rect;

    fn is_hit(&self, pos: Pos2) -> bool;
}

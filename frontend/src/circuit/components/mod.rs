use crate::circuit::ComponentId;
use eframe::emath::{Pos2, Rect};
use egui::Vec2;
use uuid::Uuid;

pub mod capacitor;
pub mod inductor;
pub mod resistor;

pub static CAPACITOR_COMPONENT_SIZE: Vec2 = Vec2::new(50.0, 25.0);
pub static INDUCTOR_COMPONENT_SIZE: Vec2 = Vec2::new(100.0, 25.0);
pub static RESISTOR_COMPONENT_SIZE: Vec2 = Vec2::new(100.0, 25.0);

#[derive(Copy, Clone)]
pub enum ComponentType {
    Resistor,
    Capacitor,
    Inductor,
}

#[derive(Copy, Clone)]
pub struct TerminalInfo {
    pub parent_component_id: ComponentId,
    pub bounds: Rect,
}

impl TerminalInfo {
    fn new(parent_component_id: ComponentId, bounds: Rect) -> Self {
        Self {
            parent_component_id,
            bounds,
        }
    }
}

#[derive(Copy, Clone)]
pub enum ComponentHitRegion {
    Terminal(TerminalInfo),
    Component(Rect),
}

pub trait Component {
    fn id(&self) -> Uuid;

    fn component_type(&self) -> ComponentType;

    fn set_value(&mut self, value: f32);

    fn value(&self) -> Option<f32>;

    fn set_position(&mut self, pos: Pos2);

    fn position(&self) -> Pos2;

    fn bounds_component(&self) -> Rect;

    // TODO: The bounds of a component should be extracted into a separate trait/funcionality
    fn bounds_terminal(&self) -> (Rect, Rect) {
        let component_bounds = self.bounds_component();

        let left_center = component_bounds.left_center();
        let right_center = component_bounds.right_center();

        let terminal_size = egui::vec2(10.0, 10.0);

        let first_terminal = Rect::from_center_size(left_center, terminal_size);
        let second_terminal = Rect::from_center_size(right_center, terminal_size);

        (first_terminal, second_terminal)
    }

    // Hit testing ordered by priority:
    // 1. Terminals
    // 2. Component
    fn hit_info(&self, pos: Pos2) -> Option<ComponentHitRegion> {
        let (first_terminal_bounds, second_terminal_bounds) = self.bounds_terminal();

        if first_terminal_bounds.contains(pos) {
            return Some(ComponentHitRegion::Terminal(TerminalInfo::new(
                self.id(),
                first_terminal_bounds,
            )));
        }

        if second_terminal_bounds.contains(pos) {
            return Some(ComponentHitRegion::Terminal(TerminalInfo::new(
                self.id(),
                second_terminal_bounds,
            )));
        }

        let component_bounds = self.bounds_component();
        if component_bounds.contains(pos) {
            return Some(ComponentHitRegion::Component(component_bounds));
        }

        None
    }
}

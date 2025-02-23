pub mod actor;
pub mod components;
pub mod store;

use eframe::emath::Pos2;
use egui::Rect;
use uuid::Uuid;

#[derive(Copy, Clone)]
pub enum ComponentType {
    Resistor,
    Capacitor,
    Inductor,
}

#[derive(Copy, Clone)]
pub enum TerminalBounds {
    First(Rect),
    Second(Rect),
}

#[derive(Copy, Clone)]
pub enum ComponentHitRegion {
    Terminal(TerminalBounds),
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
            return Some(ComponentHitRegion::Terminal(TerminalBounds::First(
                first_terminal_bounds,
            )));
        }

        if second_terminal_bounds.contains(pos) {
            return Some(ComponentHitRegion::Terminal(TerminalBounds::Second(
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

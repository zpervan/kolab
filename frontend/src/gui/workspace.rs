use crate::application::KolabApp;
use crate::circuit::actor::MoveComponentActor;
use crate::circuit::ComponentType;
use crate::circuit::{Component, ComponentHitRegion, TerminalBounds};
use crate::gui::assets::{CAPACITOR_NON_POLARISED, INDUCTOR, RESISTOR};
use eframe::epaint::{Color32, Pos2, Stroke};
use egui::{CornerRadius, CursorIcon, StrokeKind};

pub fn show(ctx: &egui::Context, app_state: &mut KolabApp) {
    let workspace_bg = egui::containers::Frame::new().fill(Color32::LIGHT_GRAY);
    let maybe_pointer_pos = ctx.pointer_interact_pos();

    egui::CentralPanel::default()
        .frame(workspace_bg)
        .show(ctx, |ui| {
            grid(ui);

            if let Some(comp) = app_state.components_store.read().pending_component() {
                draw_component(ui, comp);
            }

            for comp in app_state.components_store.read().components() {
                if let Some(pointer_pos) = maybe_pointer_pos {
                    if let Some(hit_info) = comp.hit_info(pointer_pos) {
                        let bounds = match hit_info {
                            ComponentHitRegion::Terminal(terminal_bounds) => {
                                ctx.set_cursor_icon(CursorIcon::PointingHand);
                                match terminal_bounds {
                                    TerminalBounds::First(first_bounds) => first_bounds,
                                    TerminalBounds::Second(seconds_bounds) => seconds_bounds,
                                }
                            }
                            ComponentHitRegion::Component(component_bounds) => {
                                ctx.set_cursor_icon(CursorIcon::Grab);
                                component_bounds
                            },
                        };

                        ui.painter().rect_stroke(
                            bounds,
                            CornerRadius::ZERO,
                            Stroke::new(1.0, Color32::DARK_RED),
                            StrokeKind::Outside,
                        );

                        if ui.input(|e| e.pointer.primary_pressed()) {
                            // TODO: The grabbing icon is not shown
                            // ctx.set_cursor_icon(CursorIcon::Grabbing);
                            let move_actor = Box::new(MoveComponentActor::new(
                                app_state.gui_ctx.clone(),
                                app_state.components_store.clone(),
                                comp.id(),
                            ));

                            app_state.active_actor.replace(Some(move_actor));
                        }
                    }
                }

                draw_component(ui, comp.as_ref());
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
}

// TODO: Create the grid and setup or initialization
fn grid(ui: &mut egui::Ui) {
    let canvas_size = ui.available_rect_before_wrap().scale_from_center(0.85);

    ui.painter()
        .rect_filled(canvas_size, CornerRadius::ZERO, Color32::WHITE);

    const GRID_SIZE: usize = 15;
    for x in (canvas_size.left() as i32..canvas_size.right() as i32).step_by(GRID_SIZE) {
        ui.painter().line_segment(
            [
                Pos2::new(x as f32, canvas_size.top()),
                Pos2::new(x as f32, canvas_size.bottom()),
            ],
            Stroke::new(1.0, Color32::LIGHT_GRAY),
        );
    }

    for y in (canvas_size.top() as i32..canvas_size.bottom() as i32).step_by(GRID_SIZE) {
        ui.painter().line_segment(
            [
                Pos2::new(canvas_size.left(), y as f32),
                Pos2::new(canvas_size.right(), y as f32),
            ],
            Stroke::new(1.0, Color32::LIGHT_GRAY),
        );
    }
}

fn draw_component(ui: &mut egui::Ui, component: &dyn Component) {
    let bounds = component.bounds_component();
    match component.component_type() {
        ComponentType::Resistor => {
            egui::Image::new(RESISTOR.clone()).paint_at(ui, bounds);
        }
        ComponentType::Capacitor => {
            egui::Image::new(CAPACITOR_NON_POLARISED.clone()).paint_at(ui, bounds);
        }
        ComponentType::Inductor => {
            egui::Image::new(INDUCTOR.clone()).paint_at(ui, bounds);
        }
    };
}

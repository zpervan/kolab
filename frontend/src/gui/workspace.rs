use crate::application::KolabApp;
use crate::circuit::actor::{Actor, MoveActor};
use eframe::epaint::{Color32, Pos2, Stroke};
use egui::{CornerRadius, Rect};

pub fn show(ctx: &egui::Context, app_state: &mut KolabApp) {
    let workspace_bg = egui::containers::Frame::new().fill(Color32::LIGHT_GRAY);

    // TODO: Change this to something more generic, for now it's fine
    if let Some(actor) = app_state.active_actor.get_mut() {
        if let Some(actor) = actor.downcast_ref::<MoveActor>() {
            if !actor.act() {
                log::info!("Actor is no longer active");
                actor.end();
                app_state.active_actor.replace(None);
            }
        }
    }

    egui::CentralPanel::default()
        .frame(workspace_bg)
        .show(ctx, |ui| {
            grid(ui);

            // TODO: Putting down tiles is just WIPish for testing purposes
            // Add actual component icons
            if let Some(active_actor) = app_state.active_actor.get_mut() {
                if let Some(move_actor) = active_actor.downcast_ref::<MoveActor>() {
                    if let Some(comp) = app_state
                        .components_store
                        .read()
                        .get(move_actor.component_id)
                    {
                        let min = Pos2::new(comp.position().unwrap().x, comp.position().unwrap().y);

                        let max = Pos2::new(
                            comp.position().unwrap().x + 50.0,
                            comp.position().unwrap().y + 50.0,
                        );

                        let dummy_component_size = Rect { min, max };
                        ui.painter().rect_filled(
                            dummy_component_size,
                            CornerRadius::ZERO,
                            Color32::DARK_RED,
                        );
                    }
                }
            }
            
            for comp in app_state.components_store.read().components() {
                let min = Pos2::new(comp.position().unwrap().x, comp.position().unwrap().y);

                let max = Pos2::new(
                    comp.position().unwrap().x + 50.0,
                    comp.position().unwrap().y + 50.0,
                );

                let dummy_component_size = Rect { min, max };
                ui.painter().rect_filled(
                    dummy_component_size,
                    CornerRadius::ZERO,
                    Color32::DARK_RED,
                );
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

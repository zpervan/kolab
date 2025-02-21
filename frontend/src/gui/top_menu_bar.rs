use crate::application::KolabApp;
use crate::circuit::actor::AddComponentActor;
use crate::circuit::components::capacitor::Capacitor;
use crate::circuit::components::inductor::Inductor;
use crate::circuit::components::resistor::Resistor;
use crate::circuit::{Component, ComponentType};
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;

pub fn show(ctx: &egui::Context, app_state: &mut KolabApp) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            egui::widgets::global_theme_preference_buttons(ui);
        });

        ui.horizontal(|ui| {
            add_component_button(ui, ComponentType::Resistor, ctx, app_state);
            add_component_button(ui, ComponentType::Capacitor, ctx, app_state);
            add_component_button(ui, ComponentType::Inductor, ctx, app_state);

            if ui.button("Source").clicked() {
                log::info!("TODO: Implement Source functionality");
            }

            if ui.button("Ground").clicked() {
                log::info!("TODO: Implement Ground functionality");
            }

            ui.add(egui::Separator::default().vertical());

            if ui.button("Test Ping server").clicked() {
                // TODO: Wipish just to test it
                let message_clone = app_state.message.clone();
                let ctx_clone = ctx.clone();

                spawn_local(async move {
                    if let Ok(resp) = make_http_request().await {
                        {
                            *message_clone.lock().unwrap() = resp;
                        }

                        ctx_clone.request_repaint(); // Force UI update after async completes
                    }
                });
            }

            // Scoped to unlock the mutex
            {
                ui.label(format!("{:?}", app_state.message.lock().unwrap()));
            }
        });
    });
}

pub async fn make_http_request() -> Result<String, reqwest::Error> {
    let body = reqwest::get("http://127.0.0.1:9090/").await?.text().await?;
    let json: Value = serde_json::from_str(&body).unwrap();

    println!("Received: {}", json);

    Ok(json["text"].to_string())
}

fn add_component_button(
    ui: &mut egui::Ui,
    component_type: ComponentType,
    ctx: &egui::Context,
    app_state: &mut KolabApp,
) {
    if ui.button(label(component_type)).clicked() {
        if let Some(pos) = ctx.pointer_interact_pos() {
            let component: Box<dyn Component> = match component_type {
                ComponentType::Resistor => Box::new(Resistor::new(pos)),
                ComponentType::Capacitor => Box::new(Capacitor::new(pos)),
                ComponentType::Inductor => Box::new(Inductor::new(pos)),
            };

            let actor = Box::new(AddComponentActor::new(
                app_state.gui_ctx.clone(),
                app_state.components_store.clone(),
                component.id(),
            ));

            app_state
                .components_store
                .write()
                .set_pending_component(component);

            app_state.active_actor.replace(Some(actor));
        } else {
            log::warn!("No pointer position available");
        }
    }
}

fn label(component_type: ComponentType) -> &'static str {
    match component_type {
        ComponentType::Resistor => "Resistor",
        ComponentType::Capacitor => "Capacitor",
        ComponentType::Inductor => "Inductor",
    }
}

use crate::application::KolabApp;
use crate::circuit::actor::MoveActor;
use crate::circuit::components::capacitor::Capacitor;
use crate::circuit::components::inductor::Inductor;
use crate::circuit::components::resistor::Resistor;
use crate::circuit::Component;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;

pub fn show(ctx: &egui::Context, app_state: &mut KolabApp) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            egui::widgets::global_theme_preference_buttons(ui);
        });

        ui.horizontal(|ui| {
            if ui.button("Resistor").clicked() {
                log::info!("Adding a resistor to the store");

                let resistor =
                    Box::new(Resistor::new(ctx.pointer_interact_pos().unwrap().to_vec2()));

                let actor = Box::new(MoveActor::new(
                    app_state.gui_ctx.clone(),
                    app_state.components_store.clone(),
                    resistor.id(),
                ));

                app_state
                    .components_store
                    .write()
                    .set_pending_component(resistor);

                app_state.active_actor.replace(Some(actor));
            }

            if ui.button("Capacitor").clicked() {
                log::info!("Adding a capacitor to the store");
                let capacitor = Box::new(Capacitor::new());
                app_state.components_store.write().upsert(capacitor);
            }

            if ui.button("Inductor").clicked() {
                log::info!("Adding a inductor to the store");
                let inductor = Box::new(Inductor::new());
                app_state.components_store.write().upsert(inductor);
            }

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

use std::sync::{Arc, Mutex};
use eframe::wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct KolabApp {
    label: String,

    #[serde(skip)]
    value: f32,
    message: Arc<Mutex<String>>
}

impl Default for KolabApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello from Kolab!".to_owned(),
            value: 2.7,
            message: Arc::new(Mutex::new(String::from("Waiting for message...")))
        }
    }
}

impl KolabApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for KolabApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        // Menu bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Welcome to Kolab!");

            if ui.button("Ping server").clicked() {
                // TODO: Wipish just to test it
                let message_clone = self.message.clone();
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
                ui.label(format!("From server: {:?}", self.message.lock()));
            }

            ui.separator();

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

pub async fn make_http_request() ->  Result<String, JsValue> {
    let body = reqwest::get("http://127.0.0.1:9090/")
        .await?
        .text()
        .await?;

    println!("Received: {}", body);

    Ok(body)
}

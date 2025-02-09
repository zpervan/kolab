use crate::gui;
use std::sync::{Arc, Mutex};
use wasm_bindgen_futures::spawn_local;

// TODO: Move to a common place
// TODO: Add a component store
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct KolabApp {
    label: String,

    #[serde(skip)]
    value: f32,
    pub(crate) message: Arc<Mutex<String>>,
}

impl Default for KolabApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello from Kolab!".to_owned(),
            value: 2.7,
            message: Arc::new(Mutex::new(String::from("Waiting for message..."))),
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

        gui::top_menu_bar::show(ctx, self);
        gui::workspace::show(ctx, self);
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

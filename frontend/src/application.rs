use crate::circuit::store::CircuitStore;
use crate::gui;
use egui::mutex::RwLock;
use std::any::Any;
use std::cell::Cell;
use std::sync::{Arc, Mutex};

// TODO: Move to a common place
pub struct KolabApp {
    pub components_store: Arc<RwLock<CircuitStore>>,
    pub gui_ctx: Arc<egui::Context>,
    // TODO: Should be removed, just for testing
    pub message: Arc<Mutex<String>>,
    pub active_actor: Cell<Option<Box<dyn Any>>>,
}

impl KolabApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        
        Self {
            components_store: Arc::new(RwLock::new(CircuitStore::new())),
            message: Arc::new(Mutex::new(String::from("Waiting for message..."))),
            active_actor: Cell::new(None),
            gui_ctx: Arc::new(cc.egui_ctx.clone()),
        }
    }
}

impl eframe::App for KolabApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        gui::top_menu_bar::show(ctx, self);
        gui::workspace::show(ctx, self);
    }
}

use crate::circuit::store::CircuitStore;
use egui::mutex::RwLock;
use std::cell::{Cell, RefCell};
use std::sync::Arc;
use uuid::Uuid;

pub trait Actor {
    fn begin(&self) -> bool {
        true
    }

    fn act(&self) -> bool;

    fn end(&self) -> bool {
        true
    }
}

pub struct MoveActor {
    gui_ctx: Arc<egui::Context>,
    store: Arc<RwLock<CircuitStore>>,
    pub component_id: Uuid,
    is_placing: Cell<bool>,
}

impl MoveActor {
    pub fn new(
        gui_ctx: Arc<egui::Context>,
        store: Arc<RwLock<CircuitStore>>,
        component_id: Uuid,
    ) -> Self {
        Self {
            gui_ctx,
            store,
            component_id,
            is_placing: Cell::new(false),
        }
    }
}

impl Actor for MoveActor {
    fn act(&self) -> bool {
        if self.gui_ctx.input(|i| i.pointer.primary_clicked()) {
            if !self.is_placing.get() {
                log::info!("Placing tile");
                self.is_placing.replace(true);
            } else {
                log::info!("Placing tile FINISHED");
                self.is_placing.replace(false);
                return false;
            }
        }

        let mut store = self.store.write();
        if let Some(component) = store.get_mut(&self.component_id) {
            if let Some(pointer_pos) = self.gui_ctx.pointer_interact_pos() {
                component.set_position(pointer_pos.to_vec2());
                return true;
            }
        }

        false
    }
}

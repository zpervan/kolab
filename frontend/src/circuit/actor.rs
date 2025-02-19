use crate::circuit::store::CircuitStore;
use egui::mutex::RwLock;
use log::info;
use std::cell::Cell;
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
    pub is_placing: Cell<bool>,
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
            is_placing: Cell::new(true),
        }
    }
}

impl Actor for MoveActor {
    fn act(&self) -> bool {
        if self.gui_ctx.input(|i| i.pointer.primary_clicked()) && self.is_placing.get() {
            info!("Move actor - act - Is placing: FINISHED");
            self.is_placing.replace(false);
            return false;
        }

        let mut store = self.store.write();
        if let Some(component) = store.pending_component_mut() {
            if let Some(pointer_pos) = self.gui_ctx.pointer_interact_pos() {
                component.set_position(pointer_pos.to_vec2());
                return true;
            }
        }

        false
    }

    fn end(&self) -> bool {
        info!("Move actor - end");
        let mut store = self.store.write();
        if let Some(component) = store.clear_pending_component() {
            store.upsert(component);
        }

        true
    }
}

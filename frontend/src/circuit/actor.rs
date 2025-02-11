use crate::circuit::store::CircuitStore;
use egui::mutex::RwLock;
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
        }
    }
}

impl Actor for MoveActor {
    fn act(&self) -> bool {
        log::info!("MoveActor: act");

        let mut store = self.store.write();
        if let Some(component) = store.get_mut(&self.component_id) {
            if let Some(pointer_pos) = self.gui_ctx.pointer_interact_pos() {
                component.set_position(pointer_pos.to_vec2());
                log::info!("MoveActor: moved to {:?}", component.position());

                return true;
            }
        }

        false
    }
}

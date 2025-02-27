use uuid::Uuid;
use crate::circuit::components::Component;

pub mod actor;
pub mod components;
pub mod store;

type TerminalId = Uuid;
type ComponentId = Uuid;

pub struct Circuit {
    connections: petgraph::Graph<TerminalId, ComponentId>,
}


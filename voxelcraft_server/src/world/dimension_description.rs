use crate::world::world_generator::WorldGenerator;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct DimensionDescription {
    pub name: String,
    pub id: Uuid,
    pub world_generator: Arc<dyn WorldGenerator>
}


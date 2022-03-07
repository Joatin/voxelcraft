use voxelcraft_mod::{Dimension, DEFAULT_DIMENSION_ID};
use uuid::Uuid;
use std::sync::Arc;
use voxelcraft_mod::world_generator::WorldGenerator;
use crate::world::DefaultWorldGenerator;
use std::ops::Deref;

#[derive(Debug)]
pub struct DefaultDimension {
    world_generator: Arc<dyn WorldGenerator>
}

impl DefaultDimension {
    pub fn new() -> Self {
        let world_generator = Arc::new(DefaultWorldGenerator::new());
        Self {
            world_generator
        }
    }
}

impl Dimension for DefaultDimension {
    fn id(&self) -> Uuid {
        DEFAULT_DIMENSION_ID
    }

    fn name(&self) -> &str {
        "Over World"
    }

    fn world_generator(&self) -> &dyn WorldGenerator {
        self.world_generator.deref()
    }
}
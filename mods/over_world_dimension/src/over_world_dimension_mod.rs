use crate::over_world_dimension::OverWorldDimension;
use std::sync::Arc;
use voxelcraft_id::ModId;
use voxelcraft_mod::{Block, Dimension, Mod};

pub const OVER_WORLD_DIMENSION_MOD_ID: &ModId = mod_id!("234b0b19-9037-492f-bad1-a620bd48441d");

#[derive(Debug)]
pub struct OverWorldDimensionMod {}

impl OverWorldDimensionMod {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for OverWorldDimensionMod {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl Mod for OverWorldDimensionMod {
    fn id(&self) -> &'static ModId {
        OVER_WORLD_DIMENSION_MOD_ID
    }

    fn name(&self) -> &str {
        "Over World Dimension"
    }

    async fn register_dimensions(&self, seed: u128) -> Vec<Arc<dyn Dimension>> {
        vec![Arc::new(OverWorldDimension::new(seed)) as Arc<dyn Dimension>]
    }
}

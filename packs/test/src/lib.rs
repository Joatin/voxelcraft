use std::sync::Arc;
use voxelcraft_client::setup_voxelcraft;
use voxelcraft_id::DimensionId;
use voxelcraft_mod::{Mod, ModPack};
use voxelcraft_over_world_dimension::OverWorldDimensionMod;
use voxelcraft_over_world_dimension::OVER_WORLD_DIMENSION_ID;
use voxelcraft_standard_blocks::StandardBlocksMod;

#[derive(Debug)]
pub struct TestModPack {
    mods: Vec<Arc<dyn Mod>>,
}

impl TestModPack {
    pub fn new() -> Self {
        Self {
            mods: vec![
                Arc::new(StandardBlocksMod::new()),
                Arc::new(OverWorldDimensionMod::new()),
            ],
        }
    }
}

impl ModPack for TestModPack {
    fn name(&self) -> &str {
        "Test"
    }

    fn mods(&self) -> &[Arc<dyn Mod>] {
        &self.mods
    }

    fn default_dimension(&self) -> &'static DimensionId {
        OVER_WORLD_DIMENSION_ID
    }
}

#[no_mangle]
pub extern "C" fn run_app() {
    setup_voxelcraft(TestModPack::new())
}

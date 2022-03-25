use voxelcraft_id::DimensionId;
use voxelcraft_layered_world_generator::builtin_layers::biomes::BiomesLayer;
use voxelcraft_layered_world_generator::builtin_layers::HeightLayer;
use voxelcraft_layered_world_generator::LayeredWorldGenerator;
use voxelcraft_mod::{Dimension, WorldGenerator};
use voxelcraft_standard_blocks::STONE_BLOCK_ID;

pub const OVER_WORLD_DIMENSION_ID: &DimensionId =
    dimension_id!("3300ac20-73b6-465c-9ee8-f4e71e8ef17f");

#[derive(Debug)]
pub struct OverWorldDimension {
    world_generator: LayeredWorldGenerator,
}

impl OverWorldDimension {
    pub fn new(seed: u128) -> Self {
        let world_generator = LayeredWorldGenerator::new(
            seed,
            vec![
                Box::new(BiomesLayer::new()),
                Box::new(HeightLayer::new(*STONE_BLOCK_ID, None)),
            ],
        );

        Self { world_generator }
    }
}

impl Dimension for OverWorldDimension {
    fn id(&self) -> &'static DimensionId {
        OVER_WORLD_DIMENSION_ID
    }

    fn name(&self) -> &str {
        "Over World"
    }

    fn world_generator(&self) -> &dyn WorldGenerator {
        &self.world_generator
    }
}

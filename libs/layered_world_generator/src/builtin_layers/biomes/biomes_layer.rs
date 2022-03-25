use crate::builtin_layers::biomes::biome_center_point::BiomeCenterPoint;
use crate::builtin_layers::biomes::biome_data::BiomeData;
use crate::layer::Layer;
use block_chunk::Chunk;
use noise::Worley;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use voxelcraft_core::chunk::ChunkPosition;
use voxelcraft_id::BlockId;

#[derive(Debug)]
pub struct BiomesLayer {}

impl BiomesLayer {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl Layer for BiomesLayer {
    async fn modify_chunk(
        &self,
        seed: u128,
        position: &ChunkPosition,
        chunk: &mut Chunk<BlockId, 32>,
        metadata: &mut HashMap<TypeId, Box<dyn Any + Send>>,
    ) {
        let data = BiomeData::new();
        metadata.insert(TypeId::of::<BiomeData>(), Box::new(data));
    }
}

fn get_current_zone(seed: u128, position: &ChunkPosition, grid_size: f64) {
    let x = (position.x as f64 / grid_size).floor();
    let z = (position.z as f64 / grid_size).floor();

    let center = BiomeCenterPoint::calculate_position(seed, grid_size, x as u64, z as u64);
}

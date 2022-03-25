use crate::layer::Layer;
use block_chunk::Chunk;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use voxelcraft_core::chunk::ChunkPosition;
use voxelcraft_id::BlockId;
use voxelcraft_mod::WorldGenerator;

#[derive(Debug)]
pub struct LayeredWorldGenerator {
    seed: u128,
    layers: Vec<Box<dyn Layer>>,
}

impl LayeredWorldGenerator {
    pub fn new(seed: u128, layers: Vec<Box<dyn Layer>>) -> Self {
        Self { seed, layers }
    }
}

#[async_trait::async_trait]
impl WorldGenerator for LayeredWorldGenerator {
    async fn generate_chunk(&self, position: &ChunkPosition) -> Chunk<BlockId, 32> {
        let mut chunk = Chunk::<BlockId, 32>::default();
        let mut metadata = HashMap::<TypeId, Box<dyn Any + Send>>::new();

        for layer in &self.layers {
            layer
                .modify_chunk(self.seed, position, &mut chunk, &mut metadata)
                .await;
        }

        chunk
    }
}

use block_chunk::Chunk;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::Debug;
use voxelcraft_core::chunk::ChunkPosition;
use voxelcraft_id::BlockId;

#[async_trait::async_trait]
pub trait Layer: Send + Sync + Debug {
    async fn modify_chunk(
        &self,
        seed: u128,
        position: &ChunkPosition,
        chunk: &mut Chunk<BlockId, 32>,
        metadata: &mut HashMap<TypeId, Box<dyn Any + Send>>,
    );
}

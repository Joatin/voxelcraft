use block_chunk::Chunk;
use voxelcraft_core::chunk::ChunkPosition;
use voxelcraft_id::BlockId;

#[async_trait::async_trait]
pub trait WorldGenerator: Send + Sync {
    async fn generate_chunk(&self, position: &ChunkPosition) -> Chunk<BlockId, 32>;
}

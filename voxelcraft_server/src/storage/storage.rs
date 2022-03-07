use std::error::Error;
use std::fmt::Debug;
use voxelcraft_core::chunk::ChunkPosition;

#[async_trait::async_trait]
pub trait Storage: Send + Sync + Debug {
    async fn save_chunk_blocks(
        &self,
        encoded_blocks: &[u8],
        chunk_position: ChunkPosition,
    ) -> Result<(), Box<dyn Error>>;
    async fn load_chunk_blocks(
        &self,
        chunk_position: ChunkPosition,
    ) -> Result<Vec<u8>, Box<dyn Error>>;
}

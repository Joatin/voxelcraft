use block_chunk::{Chunk, ChunkFactory};
use voxelcraft_core::chunk::ChunkPosition;

#[derive(Debug)]
pub struct DefaultWorldGenerator {}

impl DefaultWorldGenerator {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl ChunkFactory<ChunkPosition> for DefaultWorldGenerator {
    type Chunk = Chunk<u32, 32>;

    async fn generate_chunk(&self, chunk_position: &ChunkPosition) -> Self::Chunk {
        let chunk = Chunk::new();
        chunk
    }
}

use crate::Chunk;
use block_chunk::ChunkFactory;
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
    type Chunk = Chunk;

    async fn generate_chunk(&self, chunk_position: &ChunkPosition) -> Self::Chunk {
        let mut chunk = Chunk::new();
        if chunk_position.y < 0 {
            chunk.set_all(1);
        }
        chunk
    }
}

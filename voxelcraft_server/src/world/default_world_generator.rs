use voxelcraft_core::chunk::{Chunk, ChunkPosition};
use voxelcraft_mod::world_generator::WorldGenerator;

#[derive(Debug)]
pub struct DefaultWorldGenerator {}

impl DefaultWorldGenerator {
    pub fn new() -> Self {
        Self {}
    }
}

impl WorldGenerator for DefaultWorldGenerator {
    fn generate_chunk(&self, chunk_position: ChunkPosition) -> Chunk {
        let chunk = Chunk::new(chunk_position);
        chunk
    }
}

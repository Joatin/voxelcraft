use std::fmt::Debug;
use voxelcraft_core::chunk::{ChunkPosition, Chunk};

pub trait WorldGenerator: Send + Sync + Debug {
    fn generate_chunk(&self, chunk_position: ChunkPosition) -> Chunk;
}
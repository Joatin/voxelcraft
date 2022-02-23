use crate::chunk::{Chunk, ChunkPosition};
use std::fmt::Debug;

pub trait WorldGenerator: Send + Sync + Debug {
    fn generate_chunk(&self, chunk_position: ChunkPosition) -> Chunk;
}
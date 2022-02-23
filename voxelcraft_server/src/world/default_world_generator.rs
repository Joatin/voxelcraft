use crate::world::world_generator::WorldGenerator;
use crate::chunk::{Chunk, ChunkPosition};

#[derive(Debug)]
pub struct DefaultWorldGen {

}

impl DefaultWorldGen {
    pub fn new() -> Self {
        Self {}
    }
}

impl WorldGenerator for DefaultWorldGen {
    fn generate_chunk(&self, chunk_position: ChunkPosition) -> Chunk {
        todo!()
    }
}
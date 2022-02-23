use crate::block::chunk_position::ChunkPosition;

pub struct Chunk {
    position: ChunkPosition,
    world_generator: ()
}

impl Chunk {
    pub fn new(position: ChunkPosition, world_generator: ()) -> Self {
        Self {
            position,
            world_generator
        }
    }
}
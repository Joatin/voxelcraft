use crate::chunk::ChunkPosition;
use crate::entity::EntityOffset;

#[derive(Debug, Clone, PartialEq)]
pub struct EntityPosition {
    pub chunk_position: ChunkPosition,
    pub offset: EntityOffset,
}

impl EntityPosition {
    pub fn surrounding_chunks(&self, range: usize) -> Vec<ChunkPosition> {
        self.chunk_position.surrounding_chunks(range)
    }
}

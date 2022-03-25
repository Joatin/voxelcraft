use crate::chunk::ChunkPosition;
use block_chunk::BlockOffset;

pub struct BlockPosition<const SIZE: usize> {
    pub chunk_position: ChunkPosition,
    pub offset: BlockOffset<SIZE>,
}

impl<const SIZE: usize> BlockPosition<SIZE> {
    pub fn absolute_x(&self) -> i64 {
        (self.chunk_position.x as i64 * SIZE as i64) + self.offset.x as i64
    }

    pub fn absolute_y(&self) -> i64 {
        (self.chunk_position.y as i64 * SIZE as i64) + self.offset.y as i64
    }

    pub fn absolute_z(&self) -> i64 {
        (self.chunk_position.z as i64 * SIZE as i64) + self.offset.z as i64
    }
}

use crate::chunk::ChunkPosition;
use block_chunk::BlockOffset;

pub struct BlockPosition<const SIZE: usize> {
    pub chunk_position: ChunkPosition,
    pub offset: BlockOffset<SIZE>,
}

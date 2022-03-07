use crate::chunk::ChunkPosition;
use crate::block::BlockOffset;

pub struct BlockPosition {
    pub chunk_position: ChunkPosition,
    pub offset: BlockOffset
}
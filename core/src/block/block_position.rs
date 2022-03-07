use crate::block::BlockOffset;
use crate::chunk::ChunkPosition;

pub struct BlockPosition {
    pub chunk_position: ChunkPosition,
    pub offset: BlockOffset,
}

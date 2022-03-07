use crate::block::{BlockMap, BlockOffset};
use crate::chunk::ChunkPosition;
use bincode::config;
use std::error::Error;

/// The size of a chunk along all three axis
pub const CHUNK_SIZE: usize = 32;

#[derive(Debug, Clone, bincode::Encode, bincode::Decode)]
pub struct Chunk {
    blocks: BlockMap,
    chunk_position: ChunkPosition,
}

impl Chunk {
    pub fn position(&self) -> ChunkPosition {
        self.chunk_position
    }

    pub fn new(chunk_position: ChunkPosition) -> Self {
        Self {
            blocks: BlockMap::new(),
            chunk_position,
        }
    }

    pub fn block(&self, offset: BlockOffset) -> u32 {
        self.blocks.get(offset)
    }

    pub fn block_by_index(&self, index: usize) -> u32 {
        self.blocks.get_by_index(index)
    }
}

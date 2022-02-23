use crate::chunk::compressed_chunk::CompressedChunk;
use crate::block::BlockMap;
use crate::chunk::ChunkPosition;
use bincode::config;
use std::error::Error;

#[derive(Debug, Clone, bincode::Encode, bincode::Decode)]
pub struct Chunk {
    blocks: BlockMap,
    chunk_position: ChunkPosition
}

impl Chunk {

    pub fn new(chunk_position: ChunkPosition) -> Self {
        Self {
            blocks: BlockMap::new(),
            chunk_position
        }
    }

    pub fn try_into_compressed_chunk(self) -> Result<CompressedChunk, Box<dyn Error>> {
        let position = self.chunk_position;
        let encoded = bincode::encode_to_vec(self, config::standard())?;
        Ok(CompressedChunk::new(position, encoded))
    }
}
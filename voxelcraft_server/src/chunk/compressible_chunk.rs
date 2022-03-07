use crate::chunk::compressed_chunk::CompressedChunk;
use bincode::config;
use std::error::Error;
use voxelcraft_core::chunk::Chunk;

pub trait CompressibleChunk {
    fn try_into_compressed_chunk(self) -> Result<CompressedChunk, Box<dyn Error>>;
}

impl CompressibleChunk for Chunk {
    fn try_into_compressed_chunk(self) -> Result<CompressedChunk, Box<dyn Error>> {
        let position = self.position();
        let encoded = bincode::encode_to_vec(self, config::standard())?;
        Ok(CompressedChunk::new(position, encoded))
    }
}

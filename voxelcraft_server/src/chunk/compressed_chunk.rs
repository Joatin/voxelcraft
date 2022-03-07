use crate::storage::Storage;
use bincode::config;
use std::error::Error;
use std::sync::Arc;
use voxelcraft_core::chunk::{Chunk, ChunkPosition};

#[derive(Debug, Default, Clone)]
pub struct CompressedChunk {
    compressed_block_data: Vec<u8>,
    chunk_position: ChunkPosition,
}

impl CompressedChunk {
    pub(crate) fn new(chunk_position: ChunkPosition, compressed_block_data: Vec<u8>) -> Self {
        Self {
            chunk_position,
            compressed_block_data,
        }
    }

    pub async fn save(&self, storage: Arc<dyn Storage>) -> Result<(), Box<dyn Error>> {
        storage
            .save_chunk_blocks(&self.compressed_block_data, self.chunk_position)
            .await?;
        Ok(())
    }

    pub async fn load(
        storage: Arc<dyn Storage>,
        chunk_position: ChunkPosition,
    ) -> Result<Self, Box<dyn Error>> {
        let compressed_block_data = storage.load_chunk_blocks(chunk_position).await?;

        Ok(Self {
            chunk_position,
            compressed_block_data,
        })
    }

    pub fn try_into_chunk(self) -> Result<Chunk, Box<dyn Error>> {
        let (chunk, _) =
            bincode::decode_from_slice(&self.compressed_block_data, config::standard())?;
        Ok(chunk)
    }
}

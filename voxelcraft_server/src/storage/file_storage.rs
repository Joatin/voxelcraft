use crate::storage::Storage;
use std::error::Error;
use voxelcraft_core::chunk::ChunkPosition;

#[derive(Debug)]
pub struct FileStorage {}

impl FileStorage {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl Storage for FileStorage {
    async fn save_chunk_blocks(
        &self,
        _encoded_blocks: &[u8],
        _chunk_position: ChunkPosition,
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    async fn load_chunk_blocks(
        &self,
        _chunk_position: ChunkPosition,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        todo!()
    }
}

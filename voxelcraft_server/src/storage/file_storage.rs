use crate::storage::Storage;
use block_chunk::ChunkStorage;
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
impl ChunkStorage<ChunkPosition> for FileStorage {
    async fn store(
        &self,
        position: &ChunkPosition,
        bytes: Vec<u8>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        todo!()
    }

    async fn load(
        &self,
        position: &ChunkPosition,
    ) -> Result<Option<Vec<u8>>, Box<dyn Error + Send + Sync>> {
        todo!()
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

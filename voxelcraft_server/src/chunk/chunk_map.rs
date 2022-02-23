use crate::chunk::{Chunk, ChunkPosition};
use std::collections::HashMap;
use std::error::Error;
use tokio::sync::{RwLock, Mutex};
use uuid::Uuid;
use crate::world::DimensionDescription;
use std::sync::Arc;

#[derive(Debug)]
pub struct ChunkMap {
    chunks: RwLock<HashMap<ChunkPosition, Arc<Chunk>>>,
    dimensions: Arc<RwLock<HashMap<Uuid, DimensionDescription>>>
}

impl ChunkMap {
    pub fn new(dimensions: &Arc<RwLock<HashMap<Uuid, DimensionDescription>>>) -> Self {
        Self {
            chunks: RwLock::new(HashMap::new()),
            dimensions: Arc::clone(&dimensions)
        }
    }

    pub async fn get(&self, chunk_position: &ChunkPosition) -> Result<Arc<Chunk>, Box<dyn Error>> {
        let opt = {
            let lock = self.chunks.read().await;
            log::error!("{}", lock.len());
            lock.get(&chunk_position).cloned()
        };


        if let Some(chunk) = opt {
            log::info!("Chunk at {} was found in cache", chunk_position);
            Ok(chunk)
        } else {
            log::info!("Generating new chunk at {}", chunk_position);
            self.get_chunk_from_world_gen(chunk_position).await
        }
    }

    async fn get_chunk_from_world_gen(&self, chunk_position: &ChunkPosition) -> Result<Arc<Chunk>, Box<dyn Error>> {
        let dimensions = self.dimensions.read().await;
        if let Some(descriptor) = dimensions.get(&chunk_position.dimension) {
            let chunk = Arc::new(descriptor.world_generator.generate_chunk(*chunk_position));
            let mut lock = self.chunks.write().await;
            lock.insert(*chunk_position, Arc::clone(&chunk));
            log::info!("Chunk was successfully generated and stored in cache");
            Ok(chunk)
        } else {
            Err(format!("Missing descriptor for dimension {}", chunk_position.dimension).into())
        }
    }
}
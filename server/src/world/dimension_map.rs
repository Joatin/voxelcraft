use crate::Chunk;
use block_chunk::ChunkFactory;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use voxelcraft_core::chunk::ChunkPosition;
use voxelcraft_id::DimensionId;
use voxelcraft_mod::{Dimension, ModPack};

#[derive(Debug)]
pub struct DimensionMap {
    dimensions: Arc<RwLock<HashMap<DimensionId, Arc<dyn Dimension>>>>,
}

impl DimensionMap {
    pub async fn new(mod_pack: &Arc<dyn ModPack>) -> Self {
        let mut dimensions = HashMap::new();
        for module in mod_pack.mods() {
            for dim in module.register_dimensions(0).await {
                log::info!(
                    "Registering dimension: '{}', with id: '{}'",
                    dim.name(),
                    dim.id()
                );
                dimensions.insert(dim.id().clone(), dim);
            }
        }
        let dimensions = Arc::new(RwLock::new(dimensions));

        Self { dimensions }
    }
}

#[async_trait::async_trait]
impl ChunkFactory<ChunkPosition> for DimensionMap {
    type Chunk = Chunk;

    async fn generate_chunk(&self, position: &ChunkPosition) -> Self::Chunk {
        let lock = self.dimensions.read().await;
        match lock.get(&position.dimension) {
            None => {
                log::error!(
                    "A dimension for dimension id '{}' was not found!",
                    position.dimension
                );
                Self::Chunk::default()
            }
            Some(dimension) => dimension.world_generator().generate_chunk(position).await,
        }
    }
}

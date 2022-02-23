use crate::storage::Storage;
use std::sync::Arc;
use crate::chunk::{ChunkMap, ChunkPosition};
use std::error::Error;
use std::time::Duration;
use tokio::time::interval;
use std::collections::HashMap;
use crate::world::dimension_description::DimensionDescription;
use uuid::Uuid;
use tokio::sync::RwLock;
use crate::block::Block;


#[derive(Debug)]
pub struct World {
    storage: Arc<dyn Storage>,
    chunk_map: ChunkMap,
    name: String,
    players: Vec<()>,
    dimensions: Arc<RwLock<HashMap<Uuid, DimensionDescription>>>,
    block_list: Arc<HashMap<u32, Block>>
}

impl World {
    pub fn new<S: Storage + 'static>(storage: S) -> Self {
        let dimensions  = Arc::new(RwLock::new(HashMap::new()));
        let chunk_map =  ChunkMap::new(&dimensions);
        let block_list = Arc::new(HashMap::new());

        Self {
            storage: Arc::new(storage),
            chunk_map,
            name: "".to_string(),
            players: vec![],
            dimensions,
            block_list
        }
    }

    pub async fn update(&self) -> Result<(), Box<dyn Error>> {
        log::info!("Updating world");
        let chunks_to_update = self.get_chunks_that_should_update();
        log::info!("Updating {} chunks", chunks_to_update.len());
        for position in chunks_to_update {
            let _chunk = self.chunk_map.get(&position).await?;
        }

        Ok(())
    }

    fn get_chunks_that_should_update(&self) -> Vec<ChunkPosition> {
        vec![
            ChunkPosition::default()
        ]
    }

    pub fn start_update_loop(self: &Arc<Self>) {
        let world = Arc::clone(self);
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_millis(20));
            loop {
                interval.tick().await;
                world.update().await.unwrap();
            }

        });
    }
}
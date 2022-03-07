use crate::block::Block;
use crate::chunk::{ChunkMap, CompressedChunk};
use crate::dimension::DefaultDimension;
use crate::entity::Player;
use crate::event::WorldEvent;
use crate::storage::Storage;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::channel;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
use tokio::sync::{broadcast, Mutex, RwLock};
use tokio::time::interval;
use uuid::Uuid;
use voxelcraft_core::chunk::{Chunk, ChunkPosition};
use voxelcraft_core::entity::{EntityOffset, EntityPosition};
use voxelcraft_mod::{Dimension, Entity, DEFAULT_DIMENSION_ID};

#[derive(Debug)]
pub struct World {
    storage: Arc<dyn Storage>,
    chunk_map: ChunkMap,
    name: String,
    players: Mutex<HashMap<Uuid, Player>>,
    dimensions: Arc<RwLock<HashMap<Uuid, Arc<dyn Dimension>>>>,
    block_list: Arc<HashMap<u32, Block>>,
    incoming_events_receiver: Mutex<Receiver<WorldEvent>>,
    incoming_events_sender: Sender<WorldEvent>,
    outgoing_events_sender: broadcast::Sender<WorldEvent>,
}

const CHANNEL_SIZE: usize = 10_000;

impl World {
    pub fn new<S: Storage + 'static>(storage: S) -> Self {
        let dimensions = Self::construct_dimensions();
        let chunk_map = ChunkMap::new(&dimensions);
        let block_list = Arc::new(HashMap::new());

        let (incoming_events_sender, incoming_events_receiver) = channel(CHANNEL_SIZE);
        let (outgoing_events_sender, _) = broadcast::channel(CHANNEL_SIZE);

        let incoming_events_receiver = Mutex::new(incoming_events_receiver);

        Self {
            storage: Arc::new(storage),
            chunk_map,
            name: "".to_string(),
            players: Mutex::new(HashMap::new()),
            dimensions,
            block_list,
            incoming_events_sender,
            incoming_events_receiver,
            outgoing_events_sender,
        }
    }

    fn construct_dimensions() -> Arc<RwLock<HashMap<Uuid, Arc<dyn Dimension>>>> {
        let mut dimensions = HashMap::new();
        dimensions.insert(
            DEFAULT_DIMENSION_ID,
            Arc::new(DefaultDimension::new()) as Arc<dyn Dimension>,
        );
        Arc::new(RwLock::new(dimensions))
    }

    pub async fn update(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        log::debug!("Pulling new events");
        {
            let mut lock = self.incoming_events_receiver.lock().await;
            let mut events_processed = 0;
            while let Ok(_event) = lock.try_recv() {
                events_processed += 1;
            }
            log::debug!("Processed {} events", events_processed);
        }

        log::debug!("Collecting chunks to update");
        let chunks_to_update = self.get_chunks_that_should_update();

        log::debug!("Updating {} chunks", chunks_to_update.len());
        for position in chunks_to_update {
            let _chunk = self.chunk_map.get(&position).await?;
        }

        Ok(())
    }

    fn get_chunks_that_should_update(&self) -> Vec<ChunkPosition> {
        let chunk_position = ChunkPosition {
            x: 0,
            y: 0,
            z: 0,
            dimension: DEFAULT_DIMENSION_ID,
        };
        vec![chunk_position]
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

    pub fn get_event_receiver(&self) -> broadcast::Receiver<WorldEvent> {
        self.outgoing_events_sender.subscribe()
    }

    pub async fn get_chunk(
        &self,
        chunk_position: ChunkPosition,
    ) -> Result<Arc<Chunk>, Box<dyn Error + Send + Sync>> {
        self.chunk_map.get(&chunk_position).await
    }

    pub fn get_compressed_chunk(&self, _chunk_position: ChunkPosition) -> &CompressedChunk {
        todo!()
    }

    pub async fn get_player_position(&self, player_id: Uuid) -> Option<EntityPosition> {
        let players = self.players.lock().await;
        players.get(&player_id).map(|p| p.position().clone())
    }

    pub async fn load_player(&self, player_id: Uuid) {
        log::info!("Welcoming player {} to the world", player_id);
        let mut players = self.players.lock().await;
        players.insert(
            player_id,
            Player::new(
                player_id,
                EntityPosition {
                    chunk_position: ChunkPosition {
                        x: 0,
                        y: 0,
                        z: 0,
                        dimension: DEFAULT_DIMENSION_ID,
                    },
                    offset: EntityOffset {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                },
            ),
        );
    }
}

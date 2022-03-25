use crate::block::Block;
use crate::entity::Player;
use crate::event::WorldEvent;
use crate::storage::{FileStorage, Storage};
use crate::world::dimension_map::DimensionMap;
use crate::{Chunk, CHUNK_SIZE};
use block_chunk::ChunkCache;
use std::collections::HashMap;
use std::error::Error;
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::channel;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
use tokio::sync::{broadcast, MappedMutexGuard, Mutex, MutexGuard, OwnedRwLockReadGuard, RwLock};
use tokio::time::interval;
use uuid::Uuid;
use voxelcraft_core::chunk::ChunkPosition;
use voxelcraft_core::entity::{EntityOffset, EntityPosition};
use voxelcraft_id::{BlockId, DimensionId};
use voxelcraft_mod::{Dimension, Entity, ModPack};

#[derive(Debug)]
pub struct World {
    storage: Arc<dyn Storage>,
    chunk_cache: Arc<ChunkCache<ChunkPosition, BlockId, CHUNK_SIZE>>,
    name: String,
    players: Mutex<HashMap<Uuid, Player>>,
    dimensions: Arc<DimensionMap>,
    block_list: Arc<HashMap<u32, Block>>,
    incoming_events_receiver: Mutex<Receiver<WorldEvent>>,
    incoming_events_sender: Sender<WorldEvent>,
    outgoing_events_sender: broadcast::Sender<WorldEvent>,
    mod_pack: Arc<dyn ModPack>,
}

const CHANNEL_SIZE: usize = 10_000;

impl World {
    pub async fn new<S: Storage + 'static>(storage: S, mod_pack: Arc<dyn ModPack>) -> Self {
        let dimensions = Arc::new(DimensionMap::new(&mod_pack).await);
        let block_list = Arc::new(HashMap::new());
        let chunk_cache = Arc::new(ChunkCache::new(
            1000,
            1000,
            Arc::new(FileStorage::new()),
            Arc::clone(&dimensions),
        ));

        let (incoming_events_sender, incoming_events_receiver) = channel(CHANNEL_SIZE);
        let (outgoing_events_sender, _) = broadcast::channel(CHANNEL_SIZE);

        let incoming_events_receiver = Mutex::new(incoming_events_receiver);

        Self {
            storage: Arc::new(storage),
            chunk_cache,
            name: "".to_string(),
            players: Mutex::new(HashMap::new()),
            dimensions,
            block_list,
            incoming_events_sender,
            incoming_events_receiver,
            outgoing_events_sender,
            mod_pack,
        }
    }

    pub async fn update(&self, delta: f64) -> Result<(), Box<dyn Error + Send + Sync>> {
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
            self.chunk_cache
                .borrow_chunk(&position, |chunk| async move {
                    // TODO
                })
                .await?;
        }

        for player in self.players.lock().await.values_mut() {
            player.update_position(delta).await;
        }

        Ok(())
    }

    fn get_chunks_that_should_update(&self) -> Vec<ChunkPosition> {
        let chunk_position = ChunkPosition {
            x: 0,
            y: 0,
            z: 0,
            dimension: DimensionId::default(),
        };
        vec![chunk_position]
    }

    pub fn start_update_loop(self: &Arc<Self>) {
        let world = Arc::clone(self);
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_millis(16));
            let mut previous_instant = interval.tick().await;
            loop {
                let instant = interval.tick().await;

                let delta = instant.duration_since(previous_instant).as_secs_f64();

                world.update(delta).await.unwrap();

                previous_instant = instant;
            }
        });
    }

    pub fn get_event_receiver(&self) -> broadcast::Receiver<WorldEvent> {
        self.outgoing_events_sender.subscribe()
    }

    pub async fn get_chunk<
        C: Send + Sync + FnOnce(OwnedRwLockReadGuard<Chunk>) -> FR,
        FR: Future<Output = R> + Send,
        R: Send + Sync,
    >(
        &self,
        chunk_position: ChunkPosition,
        callback: C,
    ) -> Result<R, Box<dyn Error + Send + Sync>> {
        self.chunk_cache
            .borrow_chunk(&chunk_position, callback)
            .await
    }

    pub async fn get_player_position(&self, player_id: Uuid) -> Option<EntityPosition> {
        let players = self.players.lock().await;
        players.get(&player_id).map(|p| p.position().clone())
    }

    pub async fn borrow_player<
        'a,
        C: FnOnce(MappedMutexGuard<'a, Player>) -> FR,
        FR: Future<Output = R> + Send + 'a,
        R: Send + Sync,
    >(
        &'a self,
        player_id: Uuid,
        callback: C,
    ) -> Result<R, Box<dyn Error + Send + Sync>> {
        let players = self.players.lock().await;

        match MutexGuard::try_map(players, |p| match p.get_mut(&player_id) {
            None => None,
            Some(player) => Some(player),
        }) {
            Ok(player) => Ok(callback(player).await),
            Err(_) => Err("Player not found".into()),
        }
    }

    pub async fn load_player(&self, player_id: Uuid) {
        log::info!("Welcoming player {} to the world", player_id);
        let world_event_sender = self.outgoing_events_sender.clone();
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
                        dimension: self.mod_pack.default_dimension().clone(),
                    },
                    offset: EntityOffset {
                        x: 0.0,
                        y: 1.7,
                        z: 0.0,
                    },
                },
                world_event_sender,
            ),
        );
    }
}

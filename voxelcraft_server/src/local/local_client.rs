use crate::client::Client;
use crate::event::WorldEvent;
use crate::world::World;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::Mutex;
use uuid::Uuid;
use voxelcraft_core::chunk::{Chunk, ChunkPosition};
use voxelcraft_core::entity::EntityPosition;

#[derive(Debug)]
pub struct LocalClient {
    world: Arc<World>,
    player_id: Uuid,
}

impl LocalClient {
    pub fn new(world: &Arc<World>, player_id: Uuid) -> Self {
        Self {
            world: Arc::clone(&world),
            player_id,
        }
    }
}

#[async_trait::async_trait]
impl Client for LocalClient {
    async fn get_world_event_receiver(&self) -> broadcast::Receiver<WorldEvent> {
        self.world.get_event_receiver()
    }

    async fn begin_joining_world(&self) {
        self.world.load_player(self.player_id).await;
    }

    async fn join_world(&self) {
        todo!()
    }

    async fn position(&self) -> EntityPosition {
        self.world
            .get_player_position(self.player_id)
            .await
            .unwrap()
    }

    async fn get_chunk(
        &self,
        chunk_position: ChunkPosition,
    ) -> Result<Arc<Chunk>, Box<dyn Error + Send + Sync>> {
        self.world.get_chunk(chunk_position).await
    }
}

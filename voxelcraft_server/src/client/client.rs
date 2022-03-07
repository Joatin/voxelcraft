use crate::event::WorldEvent;
use tokio::sync::broadcast;
use voxelcraft_core::entity::EntityPosition;
use voxelcraft_core::chunk::{ChunkPosition, Chunk};
use std::sync::Arc;
use std::error::Error;

#[async_trait::async_trait]
pub trait Client {
    async fn get_world_event_receiver(&self) -> broadcast::Receiver<WorldEvent>;
    /// Tells the server that you are about to join the world
    async fn begin_joining_world(&self);

    /// Tells the server that you are ready to play
    async fn join_world(&self);

    /// You position
    async fn position(&self) -> EntityPosition;

    async fn get_chunk(&self, chunk_position: ChunkPosition) -> Result<Arc<Chunk>, Box<dyn Error + Send + Sync>>;
}
use crate::event::WorldEvent;
use crate::Chunk;
use std::error::Error;
use std::future::Future;
use std::sync::Arc;
use tokio::sync::{broadcast, OwnedRwLockReadGuard};
use voxelcraft_core::chunk::ChunkPosition;
use voxelcraft_core::entity::EntityPosition;

#[async_trait::async_trait]
pub trait Client {
    async fn get_world_event_receiver(&self) -> broadcast::Receiver<WorldEvent>;
    /// Tells the server that you are about to join the world
    async fn begin_joining_world(&self);

    /// Tells the server that you are ready to play
    async fn join_world(&self);

    /// You position
    async fn position(&self) -> EntityPosition;

    async fn get_chunk<
        C: Send + Sync + FnOnce(OwnedRwLockReadGuard<Chunk>) -> FR,
        FR: Future<Output = R> + Send,
        R: Send + Sync,
    >(
        &self,
        chunk_position: ChunkPosition,
        callback: C,
    ) -> Result<R, Box<dyn Error + Send + Sync>>;
}

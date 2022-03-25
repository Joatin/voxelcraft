use crate::client::Client;
use crate::event::WorldEvent;
use crate::world::World;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::{broadcast, OwnedRwLockReadGuard};

use crate::Chunk;
use cgmath::Deg;
use std::future::Future;
use uuid::Uuid;
use voxelcraft_core::chunk::ChunkPosition;
use voxelcraft_core::entity::EntityPosition;
use voxelcraft_mod::Entity;

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
            .borrow_player(
                self.player_id,
                |player| async move { player.position().clone() },
            )
            .await
            .unwrap()
    }

    async fn get_chunk<
        C: Send + Sync + FnOnce(OwnedRwLockReadGuard<Chunk>) -> FR,
        FR: Future<Output = R> + Send,
        R: Send + Sync,
    >(
        &self,
        chunk_position: ChunkPosition,
        callback: C,
    ) -> Result<R, Box<dyn Error + Send + Sync>> {
        self.world.get_chunk(chunk_position, callback).await
    }

    async fn start_move_forward(&self) {
        self.world
            .borrow_player(self.player_id, |mut player| async move {
                player.start_move_forward()
            })
            .await
            .unwrap()
    }

    async fn start_move_backward(&self) {
        self.world
            .borrow_player(self.player_id, |mut player| async move {
                player.start_move_backward()
            })
            .await
            .unwrap()
    }

    async fn start_move_right(&self) {
        self.world
            .borrow_player(self.player_id, |mut player| async move {
                player.start_move_right()
            })
            .await
            .unwrap()
    }

    async fn start_move_left(&self) {
        self.world
            .borrow_player(self.player_id, |mut player| async move {
                player.start_move_left()
            })
            .await
            .unwrap()
    }

    async fn start_jump(&self) {
        self.world
            .borrow_player(self.player_id, |mut player| async move {
                player.start_jumping()
            })
            .await
            .unwrap()
    }

    async fn start_sneak(&self) {
        self.world
            .borrow_player(self.player_id, |mut player| async move {
                player.start_sneaking()
            })
            .await
            .unwrap()
    }

    async fn stop_move_forward(&self) {
        self.world
            .borrow_player(self.player_id, |mut player| async move {
                player.stop_move_forward()
            })
            .await
            .unwrap()
    }

    async fn stop_move_backward(&self) {
        self.world
            .borrow_player(self.player_id, |mut player| async move {
                player.stop_move_backward()
            })
            .await
            .unwrap()
    }

    async fn stop_move_right(&self) {
        self.world
            .borrow_player(self.player_id, |mut player| async move {
                player.stop_move_right()
            })
            .await
            .unwrap()
    }

    async fn stop_move_left(&self) {
        self.world
            .borrow_player(self.player_id, |mut player| async move {
                player.stop_move_left()
            })
            .await
            .unwrap()
    }

    async fn stop_jump(&self) {
        self.world
            .borrow_player(
                self.player_id,
                |mut player| async move { player.stop_jumping() },
            )
            .await
            .unwrap()
    }

    async fn stop_sneak(&self) {
        self.world
            .borrow_player(self.player_id, |mut player| async move {
                player.stop_sneaking()
            })
            .await
            .unwrap()
    }

    async fn set_pitch_yaw(&self, pitch: Deg<f32>, yaw: Deg<f32>) {
        self.world
            .borrow_player(self.player_id, |mut player| async move {
                player.set_head_rotation(pitch, yaw);
            })
            .await
            .unwrap()
    }

    fn player_id(&self) -> Uuid {
        self.player_id
    }
}

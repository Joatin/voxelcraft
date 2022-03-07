use crate::local::local_client::LocalClient;
use crate::storage::FileStorage;
use crate::world::World;
use std::sync::Arc;
use uuid::Uuid;

pub fn new_local_world(_seed: u64, player_id: Uuid) -> LocalClient {
    let storage = FileStorage::new();
    let world = Arc::new(World::new(storage));
    world.start_update_loop();
    LocalClient::new(&world, player_id)
}

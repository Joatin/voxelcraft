use crate::local::local_client::LocalClient;
use crate::world::World;
use crate::storage::FileStorage;
use std::sync::Arc;

pub fn new_local_world(_seed: u64) -> LocalClient {
    let storage = FileStorage::new();
    let world = Arc::new(World::new(storage));
    world.start_update_loop();
    LocalClient::new(&world)
}
use crate::local::local_client::LocalClient;
use crate::storage::FileStorage;
use crate::world::World;
use pollster::FutureExt;
use std::sync::Arc;
use uuid::Uuid;
use voxelcraft_mod::ModPack;

pub fn new_local_world(_seed: u64, player_id: Uuid, mod_pack: &Arc<dyn ModPack>) -> LocalClient {
    let storage = FileStorage::new();
    let world = Arc::new(World::new(storage, Arc::clone(mod_pack)).block_on());
    world.start_update_loop();
    LocalClient::new(&world, player_id)
}

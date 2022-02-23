use crate::client::Client;
use crate::world::World;
use std::sync::Arc;

#[derive(Debug)]
pub struct LocalClient {
    world: Arc<World>
}

impl LocalClient {
    pub fn new(world: &Arc<World>) -> Self {
        Self {
            world: Arc::clone(&world)
        }
    }
}

impl Client for LocalClient {

}
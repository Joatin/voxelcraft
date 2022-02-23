use voxelcraft_server::local::LocalClient;
use voxelcraft_server::local::new_local_world;
use crate::game::game::Game;
use std::sync::Arc;
use crate::context::Context;
use crate::gpu::RenderContext;

#[derive(Debug)]
pub struct LocalGame {
    client: LocalClient
}


impl LocalGame {

    pub fn new() -> Self {
        let client = new_local_world(0);

        Self {
            client
        }
    }

    pub fn load() -> Self {
        todo!()
    }
}

impl Game for LocalGame {
    fn update(&self) {
        todo!()
    }

    fn render(&self, context: &Arc<Context>, render_context: &RenderContext) {
    }

    fn cleanup(&self) {
        todo!()
    }
}
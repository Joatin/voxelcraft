use crate::blocks::{Dirt, Stone};
use std::sync::Arc;
use voxelcraft_id::ModId;
use voxelcraft_mod::{Block, Mod};

pub const STANDARD_BLOCKS_MOD_ID: &ModId = mod_id!("c9817c86-141e-4bbc-b76a-d19e1aa285b7");

#[derive(Debug)]
pub struct StandardBlocksMod {}

impl StandardBlocksMod {
    pub const fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl Mod for StandardBlocksMod {
    fn id(&self) -> &'static ModId {
        STANDARD_BLOCKS_MOD_ID
    }

    fn name(&self) -> &str {
        "Standard Blocks"
    }

    async fn register_blocks(&self) -> Vec<Arc<dyn Block>> {
        vec![Arc::new(Dirt::new()), Arc::new(Stone::new())]
    }
}

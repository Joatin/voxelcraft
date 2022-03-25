use crate::block::Block;
use crate::dimension::Dimension;
use std::fmt::Debug;
use std::sync::Arc;
use voxelcraft_id::ModId;

#[async_trait::async_trait]
pub trait Mod: Send + Sync + Debug {
    fn id(&self) -> &'static ModId;
    fn name(&self) -> &str;
    async fn register_blocks(&self) -> Vec<Arc<dyn Block>> {
        vec![]
    }
    async fn register_dimensions(&self, seed: u128) -> Vec<Arc<dyn Dimension>> {
        vec![]
    }
}

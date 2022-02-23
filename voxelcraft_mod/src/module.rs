use crate::block::Block;
use crate::dimension::Dimension;
use std::sync::Arc;

#[async_trait::async_trait]
pub trait Mod: Send + Sync {
    async fn register_blocks(&self) -> Vec<Arc<dyn Block>> { vec![] }
    async fn register_dimensions(&self) -> Vec<Arc<dyn Dimension>> { vec![] }
}
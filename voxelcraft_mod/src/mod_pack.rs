use crate::block::Block;
use crate::Mod;
use std::sync::Arc;
use futures::future::join_all;

#[async_trait::async_trait]
pub trait ModPack {
    fn mods(&self) -> &[Arc<dyn Mod>];

    async fn register_blocks(&self) -> Vec<Arc<dyn Block>> {
        join_all(self.mods().iter().map(|m| async {
            m.register_blocks().await
        })).await.into_iter().flatten().collect()
    }
}
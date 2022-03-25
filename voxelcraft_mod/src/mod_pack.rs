use crate::block::Block;
use crate::{Dimension, Mod};
use futures::{stream, StreamExt};
use std::fmt::Debug;
use std::sync::Arc;
use voxelcraft_id::DimensionId;

#[async_trait::async_trait]
pub trait ModPack: Send + Sync + Debug {
    fn name(&self) -> &str;

    fn mods(&self) -> &[Arc<dyn Mod>];

    fn default_dimension(&self) -> &'static DimensionId;
}

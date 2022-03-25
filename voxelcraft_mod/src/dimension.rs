use std::fmt::Debug;

use crate::WorldGenerator;
use uuid::Uuid;
use voxelcraft_id::DimensionId;

pub trait Dimension: Send + Sync + Debug {
    fn id(&self) -> &'static DimensionId;
    fn name(&self) -> &str;
    fn world_generator(&self) -> &dyn WorldGenerator;
}

use uuid::Uuid;
use crate::world_generator::WorldGenerator;
use std::sync::Arc;
use std::fmt::Debug;

/// The id of the dimension a user will first spawn within
pub const DEFAULT_DIMENSION_ID: Uuid = Uuid::from_u128(0x9cb4cf41_5c3d_4647_83b0_8f3515da7be1);

pub trait Dimension: Send + Sync + Debug {
    fn id(&self) -> Uuid;
    fn name(&self) -> &str;
    fn world_generator(&self) -> &dyn WorldGenerator;
}
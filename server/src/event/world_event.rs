use uuid::Uuid;
use voxelcraft_core::entity::EntityPosition;

#[derive(Debug, Clone)]
pub enum WorldEvent {
    EntityPositionChanged(Uuid, EntityPosition),
}

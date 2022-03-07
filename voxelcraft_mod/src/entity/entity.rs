use voxelcraft_core::entity::EntityPosition;
use uuid::Uuid;

/// An entity is anything within the world that can be interacted with. That includes mobs and items
pub trait Entity {
    fn id(&self) -> Uuid;
    fn position(&self) -> &EntityPosition;
}
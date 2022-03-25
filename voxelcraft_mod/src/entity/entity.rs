use uuid::Uuid;
use voxelcraft_core::entity::EntityPosition;

/// An entity is anything within the world that can be interacted with. That includes mobs and items
#[async_trait::async_trait]
pub trait Entity {
    fn id(&self) -> Uuid;
    fn position(&self) -> &EntityPosition;
    async fn update_position(&mut self, delta: f64);
}

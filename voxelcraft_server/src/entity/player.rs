use voxelcraft_mod::{LivingEntity, Entity};
use uuid::Uuid;
use voxelcraft_core::entity::EntityPosition;

#[derive(Debug)]
pub struct Player {
    id: Uuid,
    position: EntityPosition
}

impl Player {
    pub fn new(id: Uuid, position: EntityPosition) -> Self {
        Self {
            id,
            position
        }
    }
}

impl Entity for Player {
    fn id(&self) -> Uuid {
        self.id
    }

    fn position(&self) -> &EntityPosition {
        &self.position
    }
}

impl LivingEntity for Player {

}


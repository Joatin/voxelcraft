use std::fmt::Debug;

use block_chunk::mesh::FaceDirection;
use mipmap::Mipmap;
use voxelcraft_id::{BlockId, FaceId, ModId};

#[async_trait::async_trait]
pub trait Block: Debug + Send + Sync {
    fn mod_id(&self) -> &'static ModId;
    fn block_id(&self) -> &'static BlockId;
    fn name(&self) -> &str;
    fn register_faces(&self) -> Vec<(&'static FaceId, &'static Mipmap<'static>)> {
        vec![]
    }
    fn get_face_for_side(&self, face_direction: &FaceDirection) -> Option<&'static FaceId> {
        None
    }
    fn is_standard_square(&self) -> bool {
        true
    }
    fn is_transparent(&self) -> bool {
        false
    }
}

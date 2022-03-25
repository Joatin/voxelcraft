use crate::STANDARD_BLOCKS_MOD_ID;
use block_chunk::mesh::FaceDirection;
use mipmap::Mipmap;
use voxelcraft_id::{BlockId, FaceId, ModId};
use voxelcraft_mod::Block;

pub const DIRT_BLOCK_ID: &BlockId = block_id!("8e69275c-6b4d-4266-a3d6-f37229d3a65f");
const DIRT_FACE_MIPMAPS: &Mipmap = &mipmap::include_mips!("dirt.png");
const DIRT_FACE_ID: &FaceId = face_id!("daabc466-b511-4695-bf4b-e55e5993a784");

#[derive(Debug)]
pub struct Dirt {}

impl Dirt {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Block for Dirt {
    fn mod_id(&self) -> &'static ModId {
        STANDARD_BLOCKS_MOD_ID
    }

    fn block_id(&self) -> &'static BlockId {
        DIRT_BLOCK_ID
    }

    fn name(&self) -> &str {
        "Dirt"
    }

    fn register_faces(&self) -> Vec<(&'static FaceId, &'static Mipmap<'static>)> {
        vec![(DIRT_FACE_ID, DIRT_FACE_MIPMAPS)]
    }

    fn get_face_for_side(&self, face_direction: &FaceDirection) -> Option<&'static FaceId> {
        Some(DIRT_FACE_ID)
    }
}

use crate::STANDARD_BLOCKS_MOD_ID;
use block_chunk::mesh::FaceDirection;
use mipmap::Mipmap;
use voxelcraft_id::{BlockId, FaceId, ModId};
use voxelcraft_mod::Block;

pub const STONE_BLOCK_ID: &BlockId = block_id!("0a311244-f6a7-4c1b-afc0-1a4ea33d9754");
const STONE_FACE_MIPMAPS: &Mipmap = &mipmap::include_mips!("stone.png");
const STONE_FACE_ID: &FaceId = face_id!("47ec204d-0253-496d-b5eb-be8aeb5c1069");

#[derive(Debug)]
pub struct Stone {}

impl Stone {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Block for Stone {
    fn mod_id(&self) -> &'static ModId {
        STANDARD_BLOCKS_MOD_ID
    }

    fn block_id(&self) -> &'static BlockId {
        STONE_BLOCK_ID
    }

    fn name(&self) -> &str {
        "Dirt"
    }

    fn register_faces(&self) -> Vec<(&'static FaceId, &'static Mipmap<'static>)> {
        vec![(STONE_FACE_ID, STONE_FACE_MIPMAPS)]
    }

    fn get_face_for_side(&self, _face_direction: &FaceDirection) -> Option<&'static FaceId> {
        Some(STONE_FACE_ID)
    }
}

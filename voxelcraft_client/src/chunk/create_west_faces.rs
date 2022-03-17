use crate::gpu::primitives::SmallTexturedArrayVertex;
use block_chunk::mesh::Face;
use voxelcraft_core::chunk::ChunkPosition;
use voxelcraft_server::CHUNK_SIZE;

pub fn create_west_faces(
    position: &ChunkPosition,
    face: &Face<u32, CHUNK_SIZE>,
) -> [SmallTexturedArrayVertex; 4] {
    let position_x = position.x as f32 * CHUNK_SIZE as f32;
    let position_y = position.y as f32 * CHUNK_SIZE as f32;
    let position_z = position.z as f32 * CHUNK_SIZE as f32;

    [
        SmallTexturedArrayVertex {
            position: [
                position_x + face.position.x as f32,
                position_y + face.position.y as f32,
                position_z + face.position.z as f32,
            ],
            tex_coords: [0.0, 0.0],
            tex_index: face.texture as i32,
        },
        SmallTexturedArrayVertex {
            position: [
                position_x + face.position.x as f32,
                position_y + face.position.y as f32 + face.height as f32,
                position_z + face.position.z as f32,
            ],
            tex_coords: [0.0, face.height as f32],
            tex_index: face.texture as i32,
        },
        SmallTexturedArrayVertex {
            position: [
                position_x + face.position.x as f32,
                position_y + face.position.y as f32 + face.height as f32,
                position_z + face.position.z as f32 + face.width as f32,
            ],
            tex_coords: [face.width as f32, face.height as f32],
            tex_index: face.texture as i32,
        },
        SmallTexturedArrayVertex {
            position: [
                position_x + face.position.x as f32,
                position_y + face.position.y as f32,
                position_z + face.position.z as f32 + face.width as f32,
            ],
            tex_coords: [face.width as f32, 0.0],
            tex_index: face.texture as i32,
        },
    ]
}

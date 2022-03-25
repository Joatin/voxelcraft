use crate::gpu::primitives::SmallTexturedArrayVertex;
use block_chunk::mesh::{Face, FaceDirection};
use voxelcraft_core::chunk::ChunkPosition;
use voxelcraft_server::CHUNK_SIZE;

pub fn create_north_faces(
    position: &ChunkPosition,
    face: &Face<i32, CHUNK_SIZE>,
) -> [SmallTexturedArrayVertex; 4] {
    assert_eq!(face.direction, FaceDirection::North);
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
                position_x + face.position.x as f32 + face.width as f32,
                position_y + face.position.y as f32 + face.height as f32,
                position_z + face.position.z as f32,
            ],
            tex_coords: [face.width as f32, face.height as f32],
            tex_index: face.texture as i32,
        },
        SmallTexturedArrayVertex {
            position: [
                position_x + face.position.x as f32 + face.width as f32,
                position_y + face.position.y as f32,
                position_z + face.position.z as f32,
            ],
            tex_coords: [face.width as f32, 0.0],
            tex_index: face.texture as i32,
        },
    ]
}

#[cfg(test)]
mod tests {
    use crate::chunk::create_north_faces::create_north_faces;
    use block_chunk::mesh::{Face, FaceDirection};
    use voxelcraft_core::chunk::ChunkPosition;

    #[test]
    fn it_should_give_correct_results() {
        let position = ChunkPosition {
            x: 1,
            y: 1,
            z: 1,
            dimension: Default::default(),
        };
        let face = Face {
            direction: FaceDirection::North,
            position: Default::default(),
            width: 1,
            height: 1,
            texture: 0,
            is_transparent: false,
        };

        let result = create_north_faces(&position, &face);

        assert_eq!(result[0].position[0], 32.0);
        assert_eq!(result[0].position[1], 32.0);
        assert_eq!(result[0].position[2], 33.0);

        assert_eq!(result[2].position[0], 33.0);
        assert_eq!(result[2].position[1], 33.0);
        assert_eq!(result[2].position[2], 33.0);
    }
}

use crate::mesh::internal::fast_mesh::handle_face::handle_face;
use crate::mesh::{BlockDescriptor, Face, FaceDirection};
use crate::{BlockOffset, Chunk};

pub fn handle_block<
    T: Send + Sync,
    TE: Send + Sync + PartialEq + Clone,
    C: Send + Sync + Fn(&T) -> Option<BlockDescriptor>,
    TEC: Send + Sync + Fn(&T, FaceDirection) -> TE,
    const SIZE: usize,
>(
    chunk: &Chunk<T, SIZE>,
    describe_callback: &C,
    texture_callback: &TEC,
    mesh: &mut Vec<Face<TE, SIZE>>,
    transparent_mesh: &mut Vec<Face<TE, SIZE>>,
    unhandled: &mut Vec<(usize, usize, usize)>,
    x: usize,
    y: usize,
    z: usize,
) {
    let position: BlockOffset<SIZE> = (x, y, z).into();

    let block = chunk.get(&position);
    if let Some(descriptor) = describe_callback(block) {
        if descriptor.is_standard_square {
            handle_face(
                chunk,
                describe_callback,
                mesh,
                transparent_mesh,
                &descriptor,
                &position,
                position.north(),
                Face::north,
                texture_callback,
                FaceDirection::North,
                block,
            );
            handle_face(
                chunk,
                describe_callback,
                mesh,
                transparent_mesh,
                &descriptor,
                &position,
                position.south(),
                Face::south,
                texture_callback,
                FaceDirection::South,
                block,
            );
            handle_face(
                chunk,
                describe_callback,
                mesh,
                transparent_mesh,
                &descriptor,
                &position,
                position.west(),
                Face::west,
                texture_callback,
                FaceDirection::West,
                block,
            );
            handle_face(
                chunk,
                describe_callback,
                mesh,
                transparent_mesh,
                &descriptor,
                &position,
                position.east(),
                Face::east,
                texture_callback,
                FaceDirection::East,
                block,
            );
            handle_face(
                chunk,
                describe_callback,
                mesh,
                transparent_mesh,
                &descriptor,
                &position,
                position.up(),
                Face::up,
                texture_callback,
                FaceDirection::Up,
                block,
            );
            handle_face(
                chunk,
                describe_callback,
                mesh,
                transparent_mesh,
                &descriptor,
                &position,
                position.down(),
                Face::down,
                texture_callback,
                FaceDirection::Down,
                block,
            );
        } else {
            unhandled.push((x, y, z));
        }
    }
}

use crate::mesh::internal::handle_face::handle_face;
use crate::mesh::{BlockDescriptor, Face};
use crate::{BlockOffset, Chunk};

pub fn handle_block<T, C: Send + Sync + Fn(&T) -> Option<BlockDescriptor>, const SIZE: usize>(
    chunk: &Chunk<T, SIZE>,
    describe_callback: &C,
    mesh: &mut Vec<Face<SIZE>>,
    transparent_mesh: &mut Vec<Face<SIZE>>,
    unhandled: &mut Vec<(usize, usize, usize)>,
    x: usize,
    y: usize,
    z: usize,
) {
    let position: BlockOffset<SIZE> = (x, y, z).into();

    if let Some(descriptor) = describe_callback(chunk.get(&position)) {
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
            );
        } else {
            unhandled.push((x, y, z));
        }
    }
}

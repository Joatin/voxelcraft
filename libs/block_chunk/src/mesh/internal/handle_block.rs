use crate::mesh::internal::handle_face::handle_face;
use crate::mesh::{BlockDescriptor, Face};
use crate::{BlockOffset, Chunk};

pub fn handle_block<
    T: PartialEq + Clone,
    C: Send + Sync + Fn(&T) -> Option<BlockDescriptor>,
    const SIZE: usize,
>(
    chunk: &Chunk<T, SIZE>,
    describe_callback: &C,
    mesh: &mut Vec<Face<T, SIZE>>,
    transparent_mesh: &mut Vec<Face<T, SIZE>>,
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
                block,
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
                block,
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
                block,
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
                block,
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
                block,
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
                block,
                &position,
                position.down(),
                Face::down,
            );
        } else {
            unhandled.push((x, y, z));
        }
    }
}

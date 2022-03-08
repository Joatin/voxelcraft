use crate::mesh::internal::push_face::push_face;
use crate::mesh::{BlockDescriptor, Face};
use crate::{BlockOffset, Chunk};

pub fn handle_face<
    T,
    FC: FnOnce(&BlockOffset<SIZE>) -> Face<SIZE>,
    C: Send + Sync + Fn(&T) -> Option<BlockDescriptor>,
    const SIZE: usize,
>(
    chunk: &Chunk<T, SIZE>,
    describe_callback: &C,
    mesh: &mut Vec<Face<SIZE>>,
    transparent_mesh: &mut Vec<Face<SIZE>>,
    descriptor: &BlockDescriptor,
    position: &BlockOffset<SIZE>,
    neighbour_position: Option<BlockOffset<SIZE>>,
    face_callback: FC,
) {
    if let Some(neighbour_position) = neighbour_position {
        if let Some(neighbour_descriptor) = describe_callback(chunk.get(&neighbour_position)) {
            if !neighbour_descriptor.is_standard_square || neighbour_descriptor.is_transparent {
                // The other block is not fully covering, push the face
                push_face(mesh, transparent_mesh, descriptor, face_callback(position));
            }
            // Otherwise do nothing
        } else {
            // Other block is AIR, we push the face
            push_face(mesh, transparent_mesh, descriptor, face_callback(position));
        }
    } else {
        // The other block is in the next chunk, lets push the face
        push_face(mesh, transparent_mesh, descriptor, face_callback(position));
    }
}

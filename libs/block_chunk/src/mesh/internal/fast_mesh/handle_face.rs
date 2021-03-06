use crate::mesh::internal::fast_mesh::push_face::push_face;
use crate::mesh::{BlockDescriptor, Face, FaceDirection};
use crate::{BlockOffset, Chunk};

pub fn handle_face<
    T: Send + Sync,
    TE: Send + Sync,
    FC: FnOnce(&BlockOffset<SIZE>, &TE, bool) -> Face<TE, SIZE>,
    C: Send + Sync + Fn(&T) -> Option<BlockDescriptor>,
    TEC: Send + Sync + Fn(&T, FaceDirection) -> TE,
    const SIZE: usize,
>(
    chunk: &Chunk<T, SIZE>,
    describe_callback: &C,
    mesh: &mut Vec<Face<TE, SIZE>>,
    transparent_mesh: &mut Vec<Face<TE, SIZE>>,
    descriptor: &BlockDescriptor,
    position: &BlockOffset<SIZE>,
    neighbour_position: Option<BlockOffset<SIZE>>,
    face_callback: FC,
    texture_callback: &TEC,
    face_direction: FaceDirection,
    block: &T,
) {
    if let Some(neighbour_position) = neighbour_position {
        if let Some(neighbour_descriptor) = describe_callback(chunk.get(&neighbour_position)) {
            if !neighbour_descriptor.is_standard_square || neighbour_descriptor.is_transparent {
                // The other block is not fully covering, push the face
                push_face(
                    mesh,
                    transparent_mesh,
                    descriptor,
                    face_callback(
                        position,
                        &texture_callback(block, face_direction),
                        descriptor.is_transparent,
                    ),
                );
            }
            // Otherwise do nothing
        } else {
            // Other block is AIR, we push the face
            push_face(
                mesh,
                transparent_mesh,
                descriptor,
                face_callback(
                    position,
                    &texture_callback(block, face_direction),
                    descriptor.is_transparent,
                ),
            );
        }
    } else {
        // The other block is in the next chunk, lets push the face
        push_face(
            mesh,
            transparent_mesh,
            descriptor,
            face_callback(
                position,
                &texture_callback(block, face_direction),
                descriptor.is_transparent,
            ),
        );
    }
}

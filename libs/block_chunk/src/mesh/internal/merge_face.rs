use crate::mesh::internal::should_create_face::should_create_face;
use crate::mesh::{BlockDescriptor, Face};
use crate::{BlockOffset, Chunk};
use std::fmt::Debug;

/// # Panics
#[inline]
pub fn merge_face<
    T: Clone + Debug + PartialEq,
    C: Send + Sync + Fn(&T) -> Option<BlockDescriptor>,
    FC: Send + Sync + Fn(&BlockOffset<SIZE>, &T) -> Face<T, SIZE>,
    const SIZE: usize,
>(
    chunk: &Chunk<T, SIZE>,
    describe_callback: &C,
    lines: &mut Vec<Face<T, SIZE>>,
    lines_transparent: &mut Vec<Face<T, SIZE>>,
    position: &BlockOffset<SIZE>,
    neighbour_position: Option<BlockOffset<SIZE>>,
    mut current_face: &mut Option<Face<T, SIZE>>,
    face_callback: FC,
) {
    let block = chunk.get(position);
    if let Some(descriptor) = describe_callback(block) {
        if should_create_face(chunk, &describe_callback, neighbour_position) {
            if let Some(face) = &mut current_face {
                assert!(face.merge_face_row(face_callback(position, block)));
            } else {
                current_face.replace(face_callback(position, block));
            }
        } else if let Some(face) = current_face.take() {
            // If next block won't have a face in this direction
            if descriptor.is_transparent {
                lines_transparent.push(face);
            } else {
                lines.push(face);
            }
        }
    } else if let Some(face) = current_face.take() {
        // If we reached air we can end the face
        if let Some(descriptor) = describe_callback(&face.block) {
            if descriptor.is_transparent {
                lines_transparent.push(face);
            } else {
                lines.push(face);
            }
        }
    }
}

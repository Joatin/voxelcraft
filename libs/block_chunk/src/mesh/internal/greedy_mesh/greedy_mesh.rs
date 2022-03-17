use crate::mesh::internal::greedy_mesh::merge_face::merge_face;
use crate::mesh::{BlockDescriptor, Face, MeshResult};
use crate::{BlockOffset, Chunk};
use std::fmt::Debug;

pub fn greedy_mesh<
    T: Sync + Send + Debug,
    TE: Sync + Send + Clone + PartialEq + Debug,
    C: Send + Sync + Fn(&T) -> Option<BlockDescriptor<TE>>,
    const SIZE: usize,
>(
    chunk: &Chunk<T, SIZE>,
    describe_callback: C,
) -> MeshResult<TE, SIZE> {
    let mut mesh = vec![];
    let mut transparent_mesh = vec![];
    let mut unhandled = vec![];

    for x in 0..SIZE {
        let mut rows = vec![];
        let mut rows_transparent = vec![];
        for y in 0..SIZE {
            let mut lines = vec![];
            let mut lines_transparent = vec![];

            let mut current_north_face = None;
            let mut current_south_face = None;
            let mut current_west_face = None;
            let mut current_east_face = None;
            let mut current_up_face = None;
            let mut current_down_face = None;

            for z in 0..SIZE {
                {
                    let position: BlockOffset<SIZE> = (x, y, z).into();
                    if describe_callback(chunk.get(&position))
                        .map_or(false, |b| !b.is_standard_square)
                    {
                        unhandled.push((x, y, z));
                    }
                }

                {
                    let position: BlockOffset<SIZE> = (z, y, x).into();
                    merge_face(
                        chunk,
                        &describe_callback,
                        &mut lines,
                        &mut lines_transparent,
                        &position,
                        position.north(),
                        &mut current_north_face,
                        Face::north,
                    );
                    merge_face(
                        chunk,
                        &describe_callback,
                        &mut lines,
                        &mut lines_transparent,
                        &position,
                        position.south(),
                        &mut current_south_face,
                        Face::south,
                    );
                }

                {
                    let position: BlockOffset<SIZE> = (x, y, z).into();
                    merge_face(
                        chunk,
                        &describe_callback,
                        &mut lines,
                        &mut lines_transparent,
                        &position,
                        position.west(),
                        &mut current_west_face,
                        Face::west,
                    );
                    merge_face(
                        chunk,
                        &describe_callback,
                        &mut lines,
                        &mut lines_transparent,
                        &position,
                        position.east(),
                        &mut current_east_face,
                        Face::east,
                    );
                }
                {
                    let position: BlockOffset<SIZE> = (z, x, y).into();
                    merge_face(
                        chunk,
                        &describe_callback,
                        &mut lines,
                        &mut lines_transparent,
                        &position,
                        position.up(),
                        &mut current_up_face,
                        Face::up,
                    );
                    merge_face(
                        chunk,
                        &describe_callback,
                        &mut lines,
                        &mut lines_transparent,
                        &position,
                        position.down(),
                        &mut current_down_face,
                        Face::down,
                    );
                }
            }

            if let Some(face) = current_north_face.take() {
                lines.push(face);
            }
            if let Some(face) = current_south_face.take() {
                lines.push(face);
            }
            if let Some(face) = current_west_face.take() {
                lines.push(face);
            }
            if let Some(face) = current_east_face.take() {
                lines.push(face);
            }
            if let Some(face) = current_up_face.take() {
                lines.push(face);
            }
            if let Some(face) = current_down_face.take() {
                lines.push(face);
            }

            for face in lines {
                if let Some(f) = rows
                    .iter_mut()
                    .find(|f: &&mut Face<TE, SIZE>| f.can_merge_column(&face))
                {
                    f.extend_face_column(&face);
                } else {
                    rows.push(face);
                }
            }

            for face in lines_transparent {
                if let Some(f) = rows_transparent
                    .iter_mut()
                    .find(|f: &&mut Face<TE, SIZE>| f.can_merge_column(&face))
                {
                    f.extend_face_column(&face);
                } else {
                    rows_transparent.push(face);
                }
            }
        }

        mesh.append(&mut rows);
        transparent_mesh.append(&mut rows_transparent);
    }

    MeshResult {
        mesh,
        transparent_mesh,
        unhandled,
    }
}

#[cfg(test)]
mod tests {
    use crate::mesh::internal::greedy_mesh;
    use crate::mesh::BlockDescriptor;
    use crate::Chunk;

    #[test]
    fn it_should_give_correct_amount_of_faces() {
        let chunk = Chunk::<usize, 8>::default();

        let result = greedy_mesh(&chunk, |_| {
            Some(BlockDescriptor::<()> {
                is_standard_square: true,
                is_transparent: false,
                texture_id: (),
            })
        });

        assert_eq!(result.mesh.len(), 6);
        assert_eq!(result.transparent_mesh.len(), 0);
        assert_eq!(result.unhandled.len(), 0);
    }

    #[test]
    fn it_should_give_correct_amount_of_faces_checker() {
        let chunk = Chunk::<usize, 8>::new_checker(0, 1);

        let result = greedy_mesh(&chunk, |val| {
            if *val == 0 {
                None
            } else {
                Some(BlockDescriptor::<()> {
                    is_standard_square: true,
                    is_transparent: false,
                    texture_id: (),
                })
            }
        });
        assert_eq!(result.mesh.len(), 1536);
        assert_eq!(result.transparent_mesh.len(), 0);
        assert_eq!(result.unhandled.len(), 0);
    }
}
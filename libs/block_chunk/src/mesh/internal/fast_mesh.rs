use crate::mesh::internal::handle_block::handle_block;
use crate::mesh::{BlockDescriptor, MeshResult};
use crate::Chunk;

pub fn fast_mesh<
    T: PartialEq + Clone,
    C: Send + Sync + Fn(&T) -> Option<BlockDescriptor>,
    const SIZE: usize,
>(
    chunk: &Chunk<T, SIZE>,
    describe_callback: C,
) -> MeshResult<T, SIZE> {
    let mut mesh = vec![];
    let mut transparent_mesh = vec![];
    let mut unhandled = vec![];

    for x in 0..SIZE {
        for y in 0..SIZE {
            for z in 0..SIZE {
                handle_block(
                    chunk,
                    &describe_callback,
                    &mut mesh,
                    &mut transparent_mesh,
                    &mut unhandled,
                    x,
                    y,
                    z,
                );
            }
        }
    }

    MeshResult {
        mesh,
        transparent_mesh,
        unhandled,
    }
}

#[cfg(test)]
mod tests {
    use crate::mesh::internal::fast_mesh;
    use crate::mesh::{BlockDescriptor, MeshableChunk};
    use crate::Chunk;

    #[tokio::test]
    async fn it_should_give_correct_amount_of_faces() {
        let chunk = Chunk::<usize, 8>::default();

        let result = chunk
            .fast_mesh(|_| {
                Some(BlockDescriptor {
                    is_standard_square: true,
                    is_transparent: false,
                })
            })
            .await;

        assert_eq!(result.mesh.len(), 8 * 8 * 6);
        assert_eq!(result.transparent_mesh.len(), 0);
        assert_eq!(result.unhandled.len(), 0);
    }

    #[test]
    fn it_should_give_correct_amount_of_faces_checker() {
        let chunk = Chunk::<usize, 8>::new_checker(0, 1);

        let result = fast_mesh(&chunk, |val| {
            if *val == 0 {
                None
            } else {
                Some(BlockDescriptor {
                    is_standard_square: true,
                    is_transparent: false,
                })
            }
        });

        assert_eq!(result.mesh.len(), 1536);
        assert_eq!(result.transparent_mesh.len(), 0);
        assert_eq!(result.unhandled.len(), 0);
    }
}

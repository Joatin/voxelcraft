use crate::mesh::internal::handle_block::handle_block;
use crate::mesh::{BlockDescriptor, MeshResult};
use crate::Chunk;

pub fn fast_mesh<T, C: Send + Sync + Fn(&T) -> Option<BlockDescriptor>, const SIZE: usize>(
    chunk: &Chunk<T, SIZE>,
    describe_callback: C,
) -> MeshResult<SIZE> {
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
}

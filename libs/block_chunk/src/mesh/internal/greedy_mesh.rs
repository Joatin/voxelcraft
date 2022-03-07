use crate::mesh::internal::fast_mesh;
use crate::mesh::internal::join_faces::join_faces;
use crate::mesh::{BlockDescriptor, MeshResult};
use crate::Chunk;

pub async fn greedy_mesh<
    T,
    C: Send + Sync + Fn(&T) -> Option<BlockDescriptor>,
    const SIZE: usize,
>(
    chunk: &Chunk<T, SIZE>,
    describe_callback: C,
) -> MeshResult<SIZE> {
    let faces = fast_mesh(chunk, describe_callback).await;

    let mesh = join_faces::<SIZE>(faces.mesh);
    let transparent_mesh = join_faces::<SIZE>(faces.transparent_mesh);

    MeshResult {
        mesh,
        transparent_mesh,
        unhandled: faces.unhandled,
    }
}

#[cfg(test)]
mod tests {
    use crate::mesh::internal::greedy_mesh;
    use crate::mesh::{BlockDescriptor, MeshableChunk};
    use crate::Chunk;

    #[tokio::test]
    async fn it_should_give_correct_amount_of_faces() {
        let chunk = Chunk::<usize, 8>::default();

        let result = greedy_mesh(&chunk, |_| {
            Some(BlockDescriptor {
                is_standard_square: true,
                is_transparent: false,
            })
        })
        .await;

        assert_eq!(result.mesh.len(), 6);
        assert_eq!(result.transparent_mesh.len(), 0);
        assert_eq!(result.unhandled.len(), 0);
    }
}

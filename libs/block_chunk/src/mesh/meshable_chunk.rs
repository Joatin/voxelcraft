use crate::{Chunk, BlockOffset};
use crate::mesh::{MeshResult, BlockDescriptor};
use crate::mesh::internal::fast_mesh;
use crate::mesh::internal::greedy_mesh;

#[async_trait::async_trait]
pub trait MeshableChunk<T, const SIZE: usize> {
    /// Only performs quick culling
    async fn fast_mesh<C: Send + Sync + Fn(&T) -> Option<BlockDescriptor>>(&self, describe_callback: C) -> MeshResult<SIZE>;

    /// Applies a greedy mesh algorithm that gives a perfect mesh, might be way slower though
    async fn greedy_mesh<C: Send + Sync + Fn(&T) -> Option<BlockDescriptor>>(&self, describe_callback: C) -> MeshResult<SIZE>;
}

#[async_trait::async_trait]
impl<T: Send + Sync + 'static, const SIZE: usize> MeshableChunk<T, SIZE> for Chunk<T, SIZE> {
    async fn fast_mesh<C: Send + Sync + Fn(&T) -> Option<BlockDescriptor>>(&self, describe_callback: C) -> MeshResult<SIZE> {
        fast_mesh(&self, describe_callback).await
    }

    async fn greedy_mesh<C: Send + Sync + Fn(&T) -> Option<BlockDescriptor>>(&self, describe_callback: C) -> MeshResult<SIZE> {
        greedy_mesh(&self, describe_callback).await
    }
}

#[cfg(test)]
mod tests {
    use crate::Chunk;
    use crate::mesh::MeshableChunk;

    #[tokio::test]
    async fn fast_mesh_should_not_panic() {
        let chunk = Chunk::<usize, 16>::default();

        let res = chunk.fast_mesh(|_block_id| {
            None
        }).await;
    }

}
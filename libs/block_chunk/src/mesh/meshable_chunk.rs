use crate::mesh::internal::fast_mesh;
use crate::mesh::internal::greedy_mesh;
use crate::mesh::{BlockDescriptor, FaceDirection, MeshResult};
use crate::Chunk;
use std::fmt::Debug;

#[async_trait::async_trait]
pub trait MeshableChunk<T: Send + Sync, TE: Send + Sync, const SIZE: usize>: Send + Sync {
    /// Only performs quick culling
    async fn fast_mesh<
        C: Send + Sync + Fn(&T) -> Option<BlockDescriptor>,
        TEC: Send + Sync + Fn(&T, FaceDirection) -> TE,
    >(
        &self,
        describe_callback: C,
        texture_callback: TEC,
    ) -> MeshResult<TE, SIZE>;

    /// Applies a greedy mesh algorithm that gives a perfect mesh, might be way slower though
    async fn greedy_mesh<
        C: Send + Sync + Fn(&T) -> Option<BlockDescriptor>,
        TEC: Send + Sync + Fn(&T, FaceDirection) -> TE,
    >(
        &self,
        describe_callback: C,
        texture_callback: TEC,
    ) -> MeshResult<TE, SIZE>;
}

#[async_trait::async_trait]
impl<
        T: Send + Sync + 'static + Debug,
        TE: 'static + Send + Sync + PartialEq + Debug + Clone,
        const SIZE: usize,
    > MeshableChunk<T, TE, SIZE> for Chunk<T, SIZE>
{
    async fn fast_mesh<
        C: Send + Sync + Fn(&T) -> Option<BlockDescriptor>,
        TEC: Send + Sync + Fn(&T, FaceDirection) -> TE,
    >(
        &self,
        describe_callback: C,
        texture_callback: TEC,
    ) -> MeshResult<TE, SIZE> {
        fast_mesh(&self, describe_callback, texture_callback)
    }

    async fn greedy_mesh<
        C: Send + Sync + Fn(&T) -> Option<BlockDescriptor>,
        TEC: Send + Sync + Fn(&T, FaceDirection) -> TE,
    >(
        &self,
        describe_callback: C,
        texture_callback: TEC,
    ) -> MeshResult<TE, SIZE> {
        greedy_mesh(&self, describe_callback, texture_callback)
    }
}

#[cfg(test)]
mod tests {
    use crate::mesh::{BlockDescriptor, MeshableChunk};
    use crate::Chunk;

    #[tokio::test]
    async fn fast_mesh_should_not_panic() {
        let chunk = Chunk::<usize, 4>::default();

        let _res = chunk
            .fast_mesh(|_block_id| Option::<BlockDescriptor>::None)
            .await;
    }
}

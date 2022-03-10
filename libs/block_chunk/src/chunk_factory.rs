use std::fmt::Debug;

#[mockall::automock(type Chunk=crate::Chunk<usize, 16>;)]
#[async_trait::async_trait]
pub trait ChunkFactory<P: 'static + Send + Sync>: Send + Sync + Debug {
    type Chunk;

    async fn generate_chunk(&self, position: &P) -> Self::Chunk;
}

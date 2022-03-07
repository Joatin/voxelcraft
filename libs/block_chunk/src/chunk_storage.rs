use std::error::Error;

#[mockall::automock]
#[async_trait::async_trait]
pub trait ChunkStorage<P: 'static + Sync + Send> {
    async fn store(&self, position: &P, bytes: Vec<u8>) -> Result<(), Box<dyn Error>>;
    async fn load(&self, position: &P) -> Result<Option<Vec<u8>>, Box<dyn Error>>;
}
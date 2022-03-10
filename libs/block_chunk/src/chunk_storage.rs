use std::error::Error;
use std::fmt::Debug;

#[mockall::automock]
#[async_trait::async_trait]
pub trait ChunkStorage<P: 'static + Sync + Send>: Send + Sync + Debug {
    async fn store(&self, position: &P, bytes: Vec<u8>)
        -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn load(&self, position: &P) -> Result<Option<Vec<u8>>, Box<dyn Error + Send + Sync>>;
}

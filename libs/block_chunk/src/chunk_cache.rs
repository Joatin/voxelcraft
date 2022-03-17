use crate::{Chunk, ChunkFactory, ChunkStorage};
use bincode::{Decode, Encode};
use std::collections::HashMap;
use std::error::Error;
use std::future::Future;
use std::hash::Hash;
use std::mem;
use std::sync::Arc;
use tokio::sync::{OwnedRwLockReadGuard, RwLock, RwLockReadGuard};

#[derive(Debug)]
pub struct ChunkCache<
    P: 'static + Hash + Eq + Send + Sync,
    T: 'static + Send + Sync,
    const SIZE: usize,
> {
    compressed_chunks: RwLock<HashMap<P, Vec<u8>>>,
    chunks: RwLock<HashMap<P, Arc<RwLock<Chunk<T, SIZE>>>>>,
    chunk_count: usize,
    max_compressed_byte_size: usize,
    storage: Arc<dyn ChunkStorage<P>>,
    factory: Box<dyn ChunkFactory<P, Chunk = Chunk<T, SIZE>>>,
}

impl<
        P: 'static + Hash + Eq + Clone + Send + Sync,
        T: 'static + Send + Sync + Encode + Decode,
        const SIZE: usize,
    > ChunkCache<P, T, SIZE>
{
    pub fn new<
        S: 'static + ChunkStorage<P>,
        F: 'static + ChunkFactory<P, Chunk = Chunk<T, SIZE>>,
    >(
        max_in_mem_chunk_byte_size: usize,
        max_compressed_byte_size: usize,
        storage: Arc<S>,
        factory: F,
    ) -> Self {
        let chunk_count = max_in_mem_chunk_byte_size / mem::size_of::<Chunk<T, SIZE>>();
        let factory = Box::new(factory);

        Self {
            compressed_chunks: RwLock::new(HashMap::new()),
            chunks: RwLock::new(HashMap::with_capacity(chunk_count)),
            chunk_count,
            max_compressed_byte_size,
            storage,
            factory,
        }
    }

    /// Borrows a chunk from the cache. The chunk can only be used for the duration of the closure
    ///
    /// # Returns
    /// The result from the closure
    ///
    /// # Errors
    /// If the storage can not load the given chunk
    pub async fn borrow_chunk<
        C: Send + Sync + FnOnce(OwnedRwLockReadGuard<Chunk<T, SIZE>>) -> FR,
        FR: Future<Output = R> + Send,
        R: Send + Sync,
    >(
        &self,
        position: &P,
        callback: C,
    ) -> Result<R, Box<dyn Error + Send + Sync>> {
        if let Some(chunk) = self.acquire_from_chunk_cache(position).await {
            let lock = chunk.read_owned().await;
            Ok(callback(lock).await)
        } else if let Some(chunk) = self.load_from_compressed_cache(position).await? {
            let lock = chunk.read_owned().await;
            Ok(callback(lock).await)
        } else if let Some(chunk) = self.load_from_storage(position).await? {
            let lock = chunk.read_owned().await;
            Ok(callback(lock).await)
        } else {
            let chunk = self.load_from_factory(position).await;
            let lock = chunk.read_owned().await;
            Ok(callback(lock).await)
        }
    }

    async fn acquire_from_chunk_cache(&self, position: &P) -> Option<Arc<RwLock<Chunk<T, SIZE>>>> {
        let lock = self.chunks.read().await;
        lock.get(position)
            .and_then(|chunk| Some(Arc::clone(&chunk)))
    }

    async fn load_from_compressed_cache(
        &self,
        position: &P,
    ) -> Result<Option<Arc<RwLock<Chunk<T, SIZE>>>>, Box<dyn Error + Send + Sync>> {
        let mut lock = self.compressed_chunks.write().await;
        if let Some(data) = lock.remove(position) {
            let chunk = Chunk::from_compressed(&data)?;
            Ok(Some(self.insert_chunk(position, chunk).await))
        } else {
            Ok(None)
        }
    }

    async fn load_from_storage(
        &self,
        position: &P,
    ) -> Result<Option<Arc<RwLock<Chunk<T, SIZE>>>>, Box<dyn Error + Send + Sync>> {
        if let Some(loaded_data) = self.storage.load(position).await? {
            let chunk = Chunk::from_compressed(&loaded_data)?;
            Ok(Some(self.insert_chunk(position, chunk).await))
        } else {
            Ok(None)
        }
    }

    async fn load_from_factory(&self, position: &P) -> Arc<RwLock<Chunk<T, SIZE>>> {
        let chunk = self.factory.generate_chunk(position).await;
        self.insert_chunk(position, chunk).await
    }

    async fn insert_chunk(
        &self,
        position: &P,
        chunk: Chunk<T, SIZE>,
    ) -> Arc<RwLock<Chunk<T, SIZE>>> {
        let mut lock = self.chunks.write().await;
        let locked_chunk = Arc::new(RwLock::new(chunk));

        // TODO: Pop if cache is full

        if lock
            .insert(position.clone(), Arc::clone(&locked_chunk))
            .is_some()
        {
            log::error!("Inserting chunk that already existed in cache... weird!");
        }
        locked_chunk
    }
}

#[cfg(test)]
mod tests {
    use crate::chunk_cache::ChunkCache;
    use crate::chunk_factory::MockChunkFactory;
    use crate::chunk_storage::MockChunkStorage;
    use crate::{BlockOffset, Chunk};
    use std::error::Error;
    use std::sync::Arc;

    #[tokio::test]
    async fn it_should_store_chunk() -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut mock_storage = MockChunkStorage::new();
        mock_storage.expect_load().returning(|_| Ok(None));

        let mut mock_factory = MockChunkFactory::<usize>::new();
        mock_factory
            .expect_generate_chunk()
            .returning(|_| Chunk::default());

        let cache = ChunkCache::new(1_000_000, 1_000_000, Arc::new(mock_storage), mock_factory);

        cache
            .borrow_chunk(&1, |chunk| async move {
                chunk.get(&BlockOffset::default());
            })
            .await?;

        Ok(())
    }
}

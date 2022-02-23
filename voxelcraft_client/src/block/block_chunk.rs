use crate::block::chunk_position::ChunkPosition;

const CHUNK_SIZE: usize = 32;

#[derive(Clone)]
pub struct BlockChunk {
    position: ChunkPosition,
    blocks: [u32; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE]
}

impl BlockChunk {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for BlockChunk {
    fn default() -> Self {
        Self {
            position: ChunkPosition::default(),
            blocks: [0; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::block::block_chunk::BlockChunk;
    use std::mem;

    #[test]
    fn block_chunk_32_should_have_correct_size() {
        assert_eq!(131084, mem::size_of::<BlockChunk>());
        // 4gb
        assert_eq!(4_295_360_512, mem::size_of::<BlockChunk>() * 32 * 32 * 32);
    }
}
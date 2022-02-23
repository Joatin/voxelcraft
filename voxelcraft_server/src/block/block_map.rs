use crate::chunk::{CHUNK_SIZE, ChunkPosition};
use serde_json::Value;
use std::sync::Arc;
use crate::storage::Storage;
use std::error::Error;
use bincode::config;



#[derive(Debug, Clone, bincode::Encode, bincode::Decode)]
pub enum BlockMap {
    AllSingleBlock(u32),
    Map(Box<[u32; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE]>)
}

impl BlockMap {
    pub fn new() -> Self {
        Self::AllSingleBlock(0)
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> u32 {
        match &self {
            BlockMap::AllSingleBlock(block) => {
                *block
            }
            BlockMap::Map(map) => {
                0
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::block::block_map::BlockMap;
    use std::mem;
    use bincode::config;
    use crate::chunk::CHUNK_SIZE;
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use std::io::Write;

    #[test]
    fn it_should_be_small_ram_mem_size_when_all_blocks_are_the_same() {
        assert_eq!(16, mem::size_of::<BlockMap>())
    }

    #[test]
    fn it_should_be_small_saved_mem_size_when_all_blocks_are_the_same() {
        let blocks = BlockMap::new();
        let encoded = bincode::encode_to_vec(blocks, config::standard()).unwrap();
        assert_eq!(2, encoded.len())
    }

    #[test]
    fn it_should_be_ok_saved_mem_size_when_all_blocks_are_different() {
        let blocks = BlockMap::Map(Box::new([1; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE]));
        let encoded = bincode::encode_to_vec(blocks, config::standard()).unwrap();
        assert_eq!(32769, encoded.len())
    }

    #[test]
    fn it_should_be_ok_saved_mem_size_when_all_blocks_are_different_gzip() {
        let blocks = BlockMap::Map(Box::new([1; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE]));
        let encoded = bincode::encode_to_vec(blocks, config::standard()).unwrap();
        let mut gz_encoder = GzEncoder::new(Vec::new(), Compression::default());
        gz_encoder.write(&encoded);
        let bytes = gz_encoder.finish().unwrap();
        assert_eq!(65, bytes.len())
    }
}
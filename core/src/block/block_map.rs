use crate::chunk::CHUNK_SIZE;
use crate::block::BlockOffset;


#[derive(Debug, Clone, bincode::Encode, bincode::Decode)]
pub enum BlockMap {
    AllSingleBlock(u32),
    Map(Box<[u32; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE]>)
}

impl BlockMap {
    pub fn new() -> Self {
        Self::AllSingleBlock(1)
    }

    pub fn get(&self, offset: BlockOffset) -> u32 {
        match &self {
            BlockMap::AllSingleBlock(block) => {
                *block
            }
            BlockMap::Map(map) => {
                map[Self::array_index(offset)]
            }
        }
    }

    pub fn get_by_index(&self, index: usize) -> u32 {
        assert!(index < CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE);
        match &self {
            BlockMap::AllSingleBlock(block) => {
                *block
            }
            BlockMap::Map(map) => {
                map[index]
            }
        }
    }

    #[inline]
    const fn array_index(offset: BlockOffset) -> usize {
        let index = (offset.z as usize * CHUNK_SIZE * CHUNK_SIZE) + (offset.y as usize * CHUNK_SIZE) + offset.x as usize;
        assert!(index < CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE);
        index
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
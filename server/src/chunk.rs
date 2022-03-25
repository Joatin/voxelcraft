use voxelcraft_id::BlockId;

pub const CHUNK_SIZE: usize = 32;

pub type Chunk = block_chunk::Chunk<BlockId, CHUNK_SIZE>;

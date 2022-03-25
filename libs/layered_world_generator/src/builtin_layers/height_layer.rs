use crate::layer::Layer;
use block_chunk::BlockOffset;
use block_chunk::Chunk;
use noise::{NoiseFn, Seedable, SuperSimplex};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use voxelcraft_core::chunk::ChunkPosition;
use voxelcraft_id::{BlockId, FaceId};

/// Fills all chunks with stone up to a certain point
#[derive(Debug)]
pub struct HeightLayer {
    replace_block: BlockId,
    fill_block: BlockId,
}

impl HeightLayer {
    /// You have to provide the block to add upp to this height, and the the block to replace. By default it will only replace AIR blocks
    pub fn new(fill_block: BlockId, replace_block: Option<BlockId>) -> Self {
        let replace_block = replace_block.unwrap_or(BlockId::AIR);
        Self {
            fill_block,
            replace_block,
        }
    }
}

#[async_trait::async_trait]
impl Layer for HeightLayer {
    async fn modify_chunk(
        &self,
        seed: u128,
        position: &ChunkPosition,
        chunk: &mut Chunk<BlockId, 32>,
        metadata: &mut HashMap<TypeId, Box<dyn Any + Send>>,
    ) {
        let generator = SuperSimplex::new().set_seed(seed as u32);

        let block_position = position.base_block_position::<32>();

        for x in 0..32 {
            for z in 0..32 {
                let height = generator.get([
                    (block_position.absolute_x() as f64 + x as f64) / 100.0,
                    (block_position.absolute_z() as f64 + z as f64) / 100.0,
                ]) * 10.0;
                for y in 0..32 {
                    if (block_position.absolute_y() + y) < height as i64 {
                        let b_pos = BlockOffset::from((x as usize, y as usize, z as usize));
                        if chunk.get(&b_pos) == &self.replace_block {
                            chunk.set(self.fill_block, &b_pos);
                        }
                    }
                }
            }
        }
    }
}

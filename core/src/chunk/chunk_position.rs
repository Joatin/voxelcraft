use crate::block::BlockPosition;
use block_chunk::BlockOffset;
use std::fmt;
use std::fmt::{Display, Formatter};
use uuid::Uuid;
use voxelcraft_id::DimensionId;

#[derive(Default, Debug, Copy, Clone, bincode::Encode, bincode::Decode, Hash, Eq, PartialEq)]
pub struct ChunkPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub dimension: DimensionId,
}

impl Display for ChunkPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "x: {}, y: {}, z: {}, dimension: {}",
            self.x, self.y, self.z, self.dimension
        )
    }
}

impl ChunkPosition {
    pub fn base_block_position<const SIZE: usize>(&self) -> BlockPosition<SIZE> {
        BlockPosition {
            chunk_position: self.clone(),
            offset: BlockOffset::default(),
        }
    }

    pub fn surrounding_chunks(&self, range: usize) -> Vec<ChunkPosition> {
        let range = range as i32;
        let start_x = self.x - range;
        let end_x = self.x + range + 1;
        let start_y = self.y - range;
        let end_y = self.y + range + 1;
        let start_z = self.z - range;
        let end_z = self.z + range + 1;

        let mut positions =
            Vec::with_capacity(((range * 2 + 1) * (range * 2 + 1) * (range * 2 + 1)) as usize);

        for x in start_x..end_x {
            for y in start_y..end_y {
                for z in start_z..end_z {
                    positions.push(ChunkPosition {
                        x,
                        y,
                        z,
                        dimension: self.dimension,
                    })
                }
            }
        }

        positions
    }
}

#[cfg(test)]
mod tests {
    use crate::chunk::ChunkPosition;

    #[test]
    fn surrounding_chunks_should_return_the_right_amount_of_chunks() {
        let position = ChunkPosition::default();
        let positions = position.surrounding_chunks(0);
        assert_eq!(1, positions.len());

        let positions = position.surrounding_chunks(16);
        assert_eq!(35_937, positions.len())
    }

    #[test]
    fn surrounding_chunks_should_contain_the_right_chunks() {
        let position = ChunkPosition::default();
        let positions = position.surrounding_chunks(16);
        assert!(positions.contains(&ChunkPosition {
            x: 0,
            y: 0,
            z: 0,
            dimension: Default::default()
        }));
        assert!(positions.contains(&ChunkPosition {
            x: -16,
            y: 0,
            z: 0,
            dimension: Default::default()
        }));
        assert!(positions.contains(&ChunkPosition {
            x: 16,
            y: 0,
            z: 0,
            dimension: Default::default()
        }));
        assert!(positions.contains(&ChunkPosition {
            x: 0,
            y: -16,
            z: 0,
            dimension: Default::default()
        }));
        assert!(positions.contains(&ChunkPosition {
            x: 0,
            y: 16,
            z: 0,
            dimension: Default::default()
        }));
        assert!(positions.contains(&ChunkPosition {
            x: 0,
            y: 0,
            z: -16,
            dimension: Default::default()
        }));
        assert!(positions.contains(&ChunkPosition {
            x: 0,
            y: 0,
            z: 16,
            dimension: Default::default()
        }));
    }
}

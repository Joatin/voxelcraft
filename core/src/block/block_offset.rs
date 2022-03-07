use crate::chunk::CHUNK_SIZE;

#[derive(Debug, Default, Copy, Clone)]
pub struct BlockOffset {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl BlockOffset {
    
    pub fn from_index(index: usize) -> Self {
        assert!(index < CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE);

        let z = index / (CHUNK_SIZE * CHUNK_SIZE);
        let index = index - (z * CHUNK_SIZE * CHUNK_SIZE);
        let y = index / CHUNK_SIZE;
        let x = index % CHUNK_SIZE;

        BlockOffset {
            x,
            y,
            z
        }
    }
    
    pub fn north(&self) -> Option<BlockOffset> {
        if self.z > 0 {
            Some(BlockOffset {
                x: self.x,
                y: self.y,
                z: self.z - 1,
            })
        } else {
            None
        }
    }
    pub fn south(&self) -> Option<BlockOffset> {
        if self.z < (CHUNK_SIZE - 1) {
            Some(BlockOffset {
                x: self.x,
                y: self.y,
                z: self.z + 1,
            })
        } else {
            None
        }
    }
    pub fn west(&self) -> Option<BlockOffset> {
        if self.x > 0 {
            Some(BlockOffset {
                x: self.x - 1,
                y: self.y,
                z: self.z,
            })
        } else {
            None
        }
    }
    pub fn east(&self) -> Option<BlockOffset> {
        if self.x < (CHUNK_SIZE - 1) {
            Some(BlockOffset {
                x: self.x + 1,
                y: self.y,
                z: self.z,
            })
        } else {
            None
        }
    }
    pub fn up(&self) -> Option<BlockOffset> {
        if self.y < (CHUNK_SIZE - 1) {
            Some(BlockOffset {
                x: self.x,
                y: self.y + 1,
                z: self.z,
            })
        } else {
            None
        }
    }
    pub fn down(&self) -> Option<BlockOffset> {
        if self.y > 0 {
            Some(BlockOffset {
                x: self.x,
                y: self.y - 1,
                z: self.z,
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::block::BlockOffset;
    use crate::chunk::CHUNK_SIZE;

    #[test]
    fn from_index_should_return_zero_offset_if_index_is_zero() {
        let offset = BlockOffset::from_index(0);

        assert_eq!(0, offset.x);
        assert_eq!(0, offset.y);
        assert_eq!(0, offset.z);
    }

    #[test]
    fn from_index_should_return_max_offset_if_index_is_max() {
        let offset = BlockOffset::from_index((CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) - 1);

        assert_eq!(CHUNK_SIZE - 1, offset.x);
        assert_eq!(CHUNK_SIZE - 1, offset.y);
        assert_eq!(CHUNK_SIZE - 1, offset.z);
    }
}
use crate::mesh::FaceDirection;
use crate::BlockOffset;
use std::error::Error;

/// Represents a block face
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Face<T, const SIZE: usize> {
    /// The direction this face is facing
    pub direction: FaceDirection,

    /// The base position for this face
    pub position: BlockOffset<SIZE>,

    /// The width of the face
    pub width: usize,

    /// The height of the face
    pub height: usize,

    pub block: T,
}

impl<T: Clone + PartialEq, const SIZE: usize> Face<T, SIZE> {
    pub fn north(position: &BlockOffset<SIZE>, block: &T) -> Self {
        Self {
            direction: FaceDirection::North,
            position: position.clone(),
            width: 1,
            height: 1,
            block: block.clone(),
        }
    }

    pub fn south(position: &BlockOffset<SIZE>, block: &T) -> Self {
        Self {
            direction: FaceDirection::South,
            position: position.clone(),
            width: 1,
            height: 1,
            block: block.clone(),
        }
    }

    pub fn west(position: &BlockOffset<SIZE>, block: &T) -> Self {
        Self {
            direction: FaceDirection::West,
            position: position.clone(),
            width: 1,
            height: 1,
            block: block.clone(),
        }
    }

    pub fn east(position: &BlockOffset<SIZE>, block: &T) -> Self {
        Self {
            direction: FaceDirection::East,
            position: position.clone(),
            width: 1,
            height: 1,
            block: block.clone(),
        }
    }

    pub fn up(position: &BlockOffset<SIZE>, block: &T) -> Self {
        Self {
            direction: FaceDirection::Up,
            position: position.clone(),
            width: 1,
            height: 1,
            block: block.clone(),
        }
    }

    pub fn down(position: &BlockOffset<SIZE>, block: &T) -> Self {
        Self {
            direction: FaceDirection::Down,
            position: position.clone(),
            width: 1,
            height: 1,
            block: block.clone(),
        }
    }

    #[must_use]
    pub fn is_single_block(&self) -> bool {
        self.is_single_height() && self.is_single_width()
    }

    #[must_use]
    pub fn is_single_height(&self) -> bool {
        self.height == 1
    }

    #[must_use]
    pub fn is_single_width(&self) -> bool {
        self.width == 1
    }

    #[must_use]
    pub fn can_merge_row(&self, other: &Self) -> bool {
        self.internal_can_merge_row(other)
    }

    fn internal_can_merge_row(&self, other: &Self) -> bool {
        let expected_position = match self.direction {
            FaceDirection::North | FaceDirection::South => BlockOffset::<SIZE> {
                x: self.position.x + self.width,
                y: self.position.y,
                z: self.position.z,
            },
            FaceDirection::West | FaceDirection::East => BlockOffset::<SIZE> {
                x: self.position.x,
                y: self.position.y,
                z: self.position.z + self.width,
            },
            FaceDirection::Up | FaceDirection::Down => BlockOffset::<SIZE> {
                x: self.position.x + self.width,
                y: self.position.y,
                z: self.position.z,
            },
        };

        other.is_single_height()
            && self.is_single_height()
            && other.position.eq(&expected_position)
            && other.direction == self.direction
            && other.block == self.block
    }

    #[must_use]
    pub fn can_merge_column(&self, other: &Self) -> bool {
        self.internal_can_merge_column(&other)
    }

    pub fn internal_can_merge_column(&self, other: &Self) -> bool {
        let expected_position = match self.direction {
            FaceDirection::North | FaceDirection::South => BlockOffset::<SIZE> {
                x: self.position.x,
                y: self.position.y + self.height,
                z: self.position.z,
            },
            FaceDirection::West | FaceDirection::East => BlockOffset::<SIZE> {
                x: self.position.x,
                y: self.position.y + self.height,
                z: self.position.z,
            },
            FaceDirection::Up | FaceDirection::Down => BlockOffset::<SIZE> {
                x: self.position.x,
                y: self.position.y,
                z: self.position.z + self.height,
            },
        };

        other.position.eq(&expected_position)
            && self.width == other.width
            && other.direction == self.direction
            && other.block == self.block
    }

    pub fn merge_face_row(&mut self, other: Self) -> bool {
        if self.internal_can_merge_row(&other) {
            self.width += other.width;
            true
        } else {
            false
        }
    }

    pub fn merge_face_column(&mut self, other: Self) -> bool {
        if self.internal_can_merge_column(&other) {
            self.height += other.height;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mesh::Face;
    use crate::BlockOffset;

    #[test]
    fn can_merge_row_north_should_work() {
        let face_1 = Face::north(&BlockOffset::<16> { x: 0, y: 0, z: 0 }, &0);
        let face_2 = Face::north(&BlockOffset::<16> { x: 1, y: 0, z: 0 }, &0);

        assert!(face_1.can_merge_row(&face_2))
    }
}

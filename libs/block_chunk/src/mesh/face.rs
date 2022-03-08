use crate::mesh::FaceDirection;
use crate::BlockOffset;
use std::error::Error;

/// Represents a block face
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Face<const SIZE: usize> {
    /// The direction this face is facing
    pub direction: FaceDirection,

    /// The base position for this face
    pub position: BlockOffset<SIZE>,

    /// The width of the face
    pub width: usize,

    /// The height of the face
    pub height: usize,
}

impl<const SIZE: usize> Face<SIZE> {
    pub fn north(position: &BlockOffset<SIZE>) -> Self {
        Self {
            direction: FaceDirection::North,
            position: position.clone(),
            width: 1,
            height: 1,
        }
    }

    pub fn south(position: &BlockOffset<SIZE>) -> Self {
        Self {
            direction: FaceDirection::South,
            position: position.clone(),
            width: 1,
            height: 1,
        }
    }

    pub fn west(position: &BlockOffset<SIZE>) -> Self {
        Self {
            direction: FaceDirection::West,
            position: position.clone(),
            width: 1,
            height: 1,
        }
    }

    pub fn east(position: &BlockOffset<SIZE>) -> Self {
        Self {
            direction: FaceDirection::East,
            position: position.clone(),
            width: 1,
            height: 1,
        }
    }

    pub fn up(position: &BlockOffset<SIZE>) -> Self {
        Self {
            direction: FaceDirection::Up,
            position: position.clone(),
            width: 1,
            height: 1,
        }
    }

    pub fn down(position: &BlockOffset<SIZE>) -> Self {
        Self {
            direction: FaceDirection::Down,
            position: position.clone(),
            width: 1,
            height: 1,
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
        self.internal_can_merge_row(other) || other.internal_can_merge_row(&self)
    }

    fn internal_can_merge_row(&self, other: &Self) -> bool {
        let expected_position = match self.direction {
            FaceDirection::North => BlockOffset::<SIZE> {
                x: self.position.x + self.width,
                y: self.position.y,
                z: self.position.z,
            },
            FaceDirection::South => BlockOffset::<SIZE> {
                x: self.position.x + self.width,
                y: self.position.y,
                z: self.position.z,
            },
            FaceDirection::West => BlockOffset::<SIZE> {
                x: self.position.x,
                y: self.position.y + self.width,
                z: self.position.z,
            },
            FaceDirection::East => BlockOffset::<SIZE> {
                x: self.position.x,
                y: self.position.y + self.width,
                z: self.position.z,
            },
            FaceDirection::Up => BlockOffset::<SIZE> {
                x: self.position.x + self.width,
                y: self.position.y,
                z: self.position.z,
            },
            FaceDirection::Down => BlockOffset::<SIZE> {
                x: self.position.x + self.width,
                y: self.position.y,
                z: self.position.z,
            },
        };

        other.is_single_height()
            && self.is_single_height()
            && other.position.eq(&expected_position)
            && other.direction == self.direction
    }

    #[must_use]
    pub fn can_merge_column(&self, other: &Self) -> bool {
        self.internal_can_merge_column(&other) || other.internal_can_merge_column(&self)
    }

    pub fn internal_can_merge_column(&self, other: &Self) -> bool {
        let expected_position = match self.direction {
            FaceDirection::North => BlockOffset::<SIZE> {
                x: self.position.x,
                y: self.position.y + self.height,
                z: self.position.z,
            },
            FaceDirection::South => BlockOffset::<SIZE> {
                x: self.position.x,
                y: self.position.y + self.height,
                z: self.position.z,
            },
            FaceDirection::West => BlockOffset::<SIZE> {
                x: self.position.x,
                y: self.position.y,
                z: self.position.z + self.height,
            },
            FaceDirection::East => BlockOffset::<SIZE> {
                x: self.position.x,
                y: self.position.y,
                z: self.position.z + self.height,
            },
            FaceDirection::Up => BlockOffset::<SIZE> {
                x: self.position.x,
                y: self.position.y,
                z: self.position.z + self.height,
            },
            FaceDirection::Down => BlockOffset::<SIZE> {
                x: self.position.x,
                y: self.position.y,
                z: self.position.z + self.height,
            },
        };

        other.position.eq(&expected_position)
            && self.width == other.width
            && other.direction == self.direction
    }

    pub fn try_merge_face_row(&mut self, other: Self) -> Result<(), Box<dyn Error>> {
        if self.internal_can_merge_row(&other) {
            self.width += other.width;
            Ok(())
        } else if other.internal_can_merge_row(&self) {
            self.position = other.position;
            self.width += other.width;
            Ok(())
        } else {
            Err("This face can't be merged".into())
        }
    }

    pub fn try_merge_face_column(&mut self, other: Self) -> Result<(), Box<dyn Error>> {
        if self.internal_can_merge_column(&other) {
            self.height += other.height;
            Ok(())
        } else if other.internal_can_merge_column(&self) {
            self.position = other.position;
            self.height += other.height;
            Ok(())
        } else {
            Err("This face can't be merged".into())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mesh::Face;
    use crate::BlockOffset;

    #[test]
    fn can_merge_row_north_should_work() {
        let face_1 = Face::north(&BlockOffset::<16> { x: 0, y: 0, z: 0 });
        let face_2 = Face::north(&BlockOffset::<16> { x: 1, y: 0, z: 0 });

        assert!(face_1.can_merge_row(&face_2))
    }
}

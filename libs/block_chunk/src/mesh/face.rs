use crate::mesh::FaceDirection;
use crate::BlockOffset;

/// Represents a block face
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Face<TE, const SIZE: usize> {
    /// The direction this face is facing
    pub direction: FaceDirection,

    /// The base position for this face
    pub position: BlockOffset<SIZE>,

    /// The width of the face
    pub width: usize,

    /// The height of the face
    pub height: usize,

    pub texture: TE,

    pub is_transparent: bool,
}

impl<TE: Clone + PartialEq, const SIZE: usize> Face<TE, SIZE> {
    pub fn north(position: &BlockOffset<SIZE>, texture: &TE, is_transparent: bool) -> Self {
        Self {
            direction: FaceDirection::North,
            position: position.clone(),
            width: 1,
            height: 1,
            texture: texture.clone(),
            is_transparent,
        }
    }

    pub fn south(position: &BlockOffset<SIZE>, texture: &TE, is_transparent: bool) -> Self {
        Self {
            direction: FaceDirection::South,
            position: position.clone(),
            width: 1,
            height: 1,
            texture: texture.clone(),
            is_transparent,
        }
    }

    pub fn west(position: &BlockOffset<SIZE>, texture: &TE, is_transparent: bool) -> Self {
        Self {
            direction: FaceDirection::West,
            position: position.clone(),
            width: 1,
            height: 1,
            texture: texture.clone(),
            is_transparent,
        }
    }

    pub fn east(position: &BlockOffset<SIZE>, texture: &TE, is_transparent: bool) -> Self {
        Self {
            direction: FaceDirection::East,
            position: position.clone(),
            width: 1,
            height: 1,
            texture: texture.clone(),
            is_transparent,
        }
    }

    pub fn up(position: &BlockOffset<SIZE>, texture: &TE, is_transparent: bool) -> Self {
        Self {
            direction: FaceDirection::Up,
            position: position.clone(),
            width: 1,
            height: 1,
            texture: texture.clone(),
            is_transparent,
        }
    }

    pub fn down(position: &BlockOffset<SIZE>, texture: &TE, is_transparent: bool) -> Self {
        Self {
            direction: FaceDirection::Down,
            position: position.clone(),
            width: 1,
            height: 1,
            texture: texture.clone(),
            is_transparent,
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
            && other.texture == self.texture
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
            && other.texture == self.texture
    }

    #[inline]
    pub fn extend_face_row(&mut self, other: &Self) {
        self.width += other.width;
    }

    #[inline]
    pub fn extend_row_by_one(&mut self) {
        self.width += 1;
    }

    #[inline]
    pub fn extend_face_column(&mut self, other: &Self) {
        self.height += other.height;
    }
}

#[cfg(test)]
mod tests {
    use crate::mesh::Face;
    use crate::BlockOffset;

    #[test]
    fn can_merge_row_north_should_work() {
        let face_1 = Face::north(&BlockOffset::<16> { x: 0, y: 0, z: 0 }, &0, false);
        let face_2 = Face::north(&BlockOffset::<16> { x: 1, y: 0, z: 0 }, &0, false);

        assert!(face_1.can_merge_row(&face_2))
    }
}

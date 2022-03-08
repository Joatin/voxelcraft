#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
pub struct BlockOffset<const SIZE: usize> {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl<const SIZE: usize> BlockOffset<SIZE> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn north(&self) -> Option<Self> {
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

    #[must_use]
    pub fn south(&self) -> Option<Self> {
        if self.z < (SIZE - 1) {
            Some(BlockOffset {
                x: self.x,
                y: self.y,
                z: self.z + 1,
            })
        } else {
            None
        }
    }

    #[must_use]
    pub fn west(&self) -> Option<Self> {
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

    #[must_use]
    pub fn east(&self) -> Option<Self> {
        if self.x < (SIZE - 1) {
            Some(BlockOffset {
                x: self.x + 1,
                y: self.y,
                z: self.z,
            })
        } else {
            None
        }
    }

    #[must_use]
    pub fn up(&self) -> Option<Self> {
        if self.y < (SIZE - 1) {
            Some(BlockOffset {
                x: self.x,
                y: self.y + 1,
                z: self.z,
            })
        } else {
            None
        }
    }

    #[must_use]
    pub fn down(&self) -> Option<Self> {
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

impl<const SIZE: usize> From<(usize, usize, usize)> for BlockOffset<SIZE> {
    fn from(pos: (usize, usize, usize)) -> Self {
        assert!(pos.0 < SIZE);
        assert!(pos.1 < SIZE);
        assert!(pos.2 < SIZE);

        Self {
            x: pos.0,
            y: pos.1,
            z: pos.2,
        }
    }
}

impl<const SIZE: usize> From<&(usize, usize, usize)> for BlockOffset<SIZE> {
    fn from(pos: &(usize, usize, usize)) -> Self {
        assert!(pos.0 < SIZE);
        assert!(pos.1 < SIZE);
        assert!(pos.2 < SIZE);

        Self {
            x: pos.0,
            y: pos.1,
            z: pos.2,
        }
    }
}

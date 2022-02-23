use uuid::Uuid;
use std::fmt;

#[derive(Default, Debug, Copy, Clone, bincode::Encode, bincode::Decode, Hash, Eq, PartialEq)]
pub struct ChunkPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    #[bincode(with_serde)]
    pub dimension: Uuid,
}

impl ChunkPosition {

}

impl fmt::Display for ChunkPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}, z: {}, dimension: {}", self.x, self.y, self.z, self.dimension)
    }
}
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    Eq,
    PartialEq,
    Hash,
    Ord,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    bincode::Encode,
    bincode::Decode,
)]
pub struct BlockId(u128);

impl BlockId {
    pub const AIR: Self = Self(0);
    pub const DEBUG: Self = Self(1);

    pub const fn from_u128(num: u128) -> Self {
        Self(num)
    }
}

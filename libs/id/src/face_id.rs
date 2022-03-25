#[derive(
    Debug,
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
pub struct FaceId(u128);

impl FaceId {
    pub const DEBUG: Self = Self(0);

    pub const fn from_u128(num: u128) -> Self {
        Self(num)
    }
}

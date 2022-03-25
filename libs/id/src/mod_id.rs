#[derive(
    Debug,
    Clone,
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
pub struct ModId(u128);

impl ModId {
    pub const fn from_u128(num: u128) -> Self {
        Self(num)
    }
}

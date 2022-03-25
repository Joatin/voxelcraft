use std::fmt::{Display, Formatter};

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
pub struct DimensionId(u128);

impl DimensionId {
    pub const fn from_u128(num: u128) -> Self {
        Self(num)
    }
}

impl Display for DimensionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

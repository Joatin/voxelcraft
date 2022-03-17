use std::ops::Deref;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct FaceId(Uuid);

impl Deref for FaceId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

use std::ops::Deref;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ModId(Uuid);

impl Deref for ModId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

use std::fmt::Debug;
use std::sync::Arc;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait Block: Debug + Send + Sync {
    fn mod_id(&self) -> Uuid;
    fn block_id(&self) -> u16;
    fn name(&self) -> &str;
}

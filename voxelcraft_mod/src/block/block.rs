use uuid::Uuid;
use std::sync::Arc;
use std::fmt::Debug;

#[async_trait::async_trait]
pub trait Block: Debug + Send + Sync {
    fn mod_id(&self) -> Uuid;
    fn block_id(&self) -> u16;
    fn name(&self) -> &str;
}
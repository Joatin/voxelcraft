use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Block {
    name: String,
    id: Uuid,
}

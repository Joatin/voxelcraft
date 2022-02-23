use uuid::Uuid;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Block {
    name: String,
    id: Uuid
}
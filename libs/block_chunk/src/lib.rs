//! This library helps managing 3 dimensional arrays of blocks, also called chunks. It has some
//! utility functions that helps make life easier

#![warn(clippy::all)]
#![warn(clippy::correctness)]
#![warn(clippy::suspicious)]
#![warn(clippy::style)]
#![warn(clippy::complexity)]
#![warn(clippy::perf)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::module_inception)]

pub use self::block_offset::BlockOffset;
pub use self::chunk::Chunk;
pub use self::chunk_cache::ChunkCache;
pub use self::chunk_factory::ChunkFactory;
pub use self::chunk_storage::ChunkStorage;

mod block_offset;
mod chunk;
mod chunk_cache;
mod chunk_factory;
mod chunk_storage;

#[cfg(feature = "mesh")]
pub mod mesh;

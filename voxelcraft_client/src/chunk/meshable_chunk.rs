use crate::chunk::ChunkMesh;
use voxelcraft_core::chunk::Chunk;
use wgpu::Device;

#[async_trait::async_trait]
pub trait MeshableChunk {
    async fn create_mesh(&self, device: &Device) -> ChunkMesh;
}

#[async_trait::async_trait]
impl MeshableChunk for Chunk {
    async fn create_mesh(&self, device: &Device) -> ChunkMesh {
        ChunkMesh::new(device, &self).await
    }
}

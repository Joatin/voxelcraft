use crate::gpu::primitives::TexturedArrayVertex;
use crate::gpu::RenderContext;
use block_chunk::mesh::{BlockDescriptor, MeshableChunk};
use smallvec::SmallVec;
use std::mem;
use std::time::Instant;
use voxelcraft_core::chunk::ChunkPosition;
use voxelcraft_server::Chunk;
use wgpu::{Buffer, BufferDescriptor, BufferUsages, Device, IndexFormat, RenderPass};

#[derive(Debug)]
pub struct ChunkMesh {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    index_count: u32,
}

impl ChunkMesh {
    pub async fn new(device: &Device, chunk: &Chunk, position: &ChunkPosition) -> Self {
        log::debug!("Building mesh for chunk at: {}", position);
        let start_time = Instant::now();
        let mesh = chunk
            .greedy_mesh(|id| {
                Some(BlockDescriptor {
                    is_standard_square: false,
                    is_transparent: false,
                })
            })
            .await;

        let indices = Self::get_indexes(&mesh);
        log::info!(
            "Collected {} faces in {:?} for chunk at: {}",
            mesh.len(),
            Instant::now().duration_since(start_time),
            position
        );

        let vertex_buffer = {
            let buffer = device.create_buffer(&BufferDescriptor {
                label: Some(&format!("Chunk vertex buffer at: {}", position)),
                size: (mem::size_of::<TexturedArrayVertex>() * mesh.len()) as u64,
                usage: BufferUsages::VERTEX,
                mapped_at_creation: true,
            });

            let slice = buffer.slice(..);
            slice
                .get_mapped_range_mut()
                .copy_from_slice(bytemuck::cast_slice(&mesh));

            buffer.unmap();

            buffer
        };

        let index_buffer = {
            let buffer = device.create_buffer(&BufferDescriptor {
                label: Some(&format!("Chunk index buffer at: {}", chunk.position())),
                size: (mem::size_of::<u32>() * (mesh.len() / 4) * 6) as u64,
                usage: BufferUsages::INDEX,
                mapped_at_creation: true,
            });

            let slice = buffer.slice(..);
            slice
                .get_mapped_range_mut()
                .copy_from_slice(bytemuck::cast_slice(&indices));

            buffer.unmap();

            buffer
        };

        let index_count = indices.len() as u32;

        Self {
            vertex_buffer,
            index_buffer,
            index_count,
        }
    }

    fn get_indexes(vertices: &Vec<TexturedArrayVertex>) -> Vec<u32> {
        let mut indices = Vec::with_capacity(vertices.len());
        for n in 0..(vertices.len() / 4) {
            let offset = n * 4;
            indices.push((0 + offset) as u32);
            indices.push((1 + offset) as u32);
            indices.push((2 + offset) as u32);
            indices.push((3 + offset) as u32);
            indices.push((2 + offset) as u32);
            indices.push((1 + offset) as u32);
        }

        indices
    }

    pub fn render<'a>(
        &'a mut self,
        _render_context: &RenderContext,
        render_pass: &mut RenderPass<'a>,
    ) {
        render_pass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint32);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw_indexed(0..self.index_count, 0, 0..1);
    }
}

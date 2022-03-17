use crate::chunk::create_down_faces::create_down_faces;
use crate::chunk::create_east_faces::create_east_faces;
use crate::chunk::create_north_faces::create_north_faces;
use crate::chunk::create_south_faces::create_south_faces;
use crate::chunk::create_up_faces::create_up_faces;
use crate::chunk::create_west_faces::create_west_faces;
use crate::gpu::primitives::{SmallTexturedArrayVertex, TexturedArrayVertex};
use crate::gpu::RenderContext;
use block_chunk::mesh::{BlockDescriptor, Face, FaceDirection, MeshableChunk};
use smallvec::SmallVec;
use std::error::Error;
use std::mem;
use std::sync::Arc;
use std::time::Instant;
use voxelcraft_core::chunk::ChunkPosition;
use voxelcraft_server::Chunk;
use voxelcraft_server::CHUNK_SIZE;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{Buffer, BufferDescriptor, BufferUsages, Device, IndexFormat, RenderPass};

#[derive(Debug)]
pub struct ChunkMesh {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    index_count: u32,
}

impl ChunkMesh {
    pub async fn new(
        device: &Arc<Device>,
        chunk: &Chunk,
        position: &ChunkPosition,
    ) -> Result<Self, Box<dyn Error + Send + Sync>> {
        log::debug!("Building mesh for chunk at: {}", position);
        let start_time = Instant::now();
        let meshes = chunk
            .greedy_mesh(|id| {
                if *id != 0 {
                    Some(BlockDescriptor {
                        is_standard_square: true,
                        is_transparent: false,
                        texture_id: 0,
                    })
                } else {
                    None
                }
            })
            .await;

        let device = Arc::clone(&device);
        let position = *position;
        let handle = tokio::task::spawn_blocking(move || {
            let (mesh, mesh_indices) = Self::convert_mesh(&position, meshes.mesh);

            let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
                label: Some(&format!("Chunk vertex buffer at: {}", position)),
                contents: bytemuck::cast_slice(&mesh),
                usage: BufferUsages::VERTEX,
            });

            let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
                label: Some(&format!("Chunk index buffer at: {}", position)),
                contents: bytemuck::cast_slice(&mesh_indices),
                usage: BufferUsages::INDEX,
            });
            let index_count = mesh_indices.len() as u32;

            (vertex_buffer, index_buffer, index_count)
        });

        let (vertex_buffer, index_buffer, index_count) = handle.await?;

        Ok(Self {
            vertex_buffer,
            index_buffer,
            index_count,
        })
    }

    pub fn render<'a>(
        &'a mut self,
        _render_context: &RenderContext,
        render_pass: &mut RenderPass<'a>,
    ) {
        if self.index_count > 0 {
            render_pass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint32);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw_indexed(0..self.index_count, 0, 0..1);
        }
    }

    fn convert_mesh(
        position: &ChunkPosition,
        faces: Vec<Face<u32, CHUNK_SIZE>>,
    ) -> (Vec<SmallTexturedArrayVertex>, Vec<u32>) {
        let mut indices = vec![];
        let mut vertices = vec![];

        for (index, face) in faces.iter().enumerate() {
            match face.direction {
                FaceDirection::North => {
                    vertices.extend_from_slice(&create_north_faces(position, face));

                    let offset = (indices.len() / 6) * 4;

                    indices.push((1 + offset) as u32);
                    indices.push((2 + offset) as u32);
                    indices.push((3 + offset) as u32);
                    indices.push((0 + offset) as u32);
                    indices.push((1 + offset) as u32);
                    indices.push((3 + offset) as u32);
                }
                FaceDirection::South => {
                    vertices.extend_from_slice(&create_south_faces(position, face));

                    let offset = (indices.len() / 6) * 4;

                    indices.push((3 + offset) as u32);
                    indices.push((2 + offset) as u32);
                    indices.push((1 + offset) as u32);
                    indices.push((3 + offset) as u32);
                    indices.push((1 + offset) as u32);
                    indices.push((0 + offset) as u32);
                }
                FaceDirection::West => {
                    vertices.extend_from_slice(&create_west_faces(position, face));

                    let offset = (indices.len() / 6) * 4;

                    indices.push((1 + offset) as u32);
                    indices.push((2 + offset) as u32);
                    indices.push((3 + offset) as u32);
                    indices.push((0 + offset) as u32);
                    indices.push((1 + offset) as u32);
                    indices.push((3 + offset) as u32);
                }
                FaceDirection::East => {
                    vertices.extend_from_slice(&create_east_faces(position, face));

                    let offset = (indices.len() / 6) * 4;

                    indices.push((1 + offset) as u32);
                    indices.push((2 + offset) as u32);
                    indices.push((3 + offset) as u32);
                    indices.push((0 + offset) as u32);
                    indices.push((1 + offset) as u32);
                    indices.push((3 + offset) as u32);
                }
                FaceDirection::Up => {
                    vertices.extend_from_slice(&create_up_faces(position, face));

                    let offset = (indices.len() / 6) * 4;

                    indices.push((1 + offset) as u32);
                    indices.push((2 + offset) as u32);
                    indices.push((3 + offset) as u32);
                    indices.push((0 + offset) as u32);
                    indices.push((1 + offset) as u32);
                    indices.push((3 + offset) as u32);
                }
                FaceDirection::Down => {
                    vertices.extend_from_slice(&create_down_faces(position, face));

                    let offset = (indices.len() / 6) * 4;

                    indices.push((3 + offset) as u32);
                    indices.push((2 + offset) as u32);
                    indices.push((1 + offset) as u32);
                    indices.push((3 + offset) as u32);
                    indices.push((1 + offset) as u32);
                    indices.push((0 + offset) as u32);
                }
            }
        }

        (vertices, indices)
    }
}

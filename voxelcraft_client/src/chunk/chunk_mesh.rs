use wgpu::{Buffer, Device, BufferDescriptor, BufferUsages, RenderPass, IndexFormat};
use voxelcraft_core::chunk::{Chunk, CHUNK_SIZE};
use crate::gpu::RenderContext;
use voxelcraft_core::block::BlockOffset;
use crate::gpu::primitives::TexturedArrayVertex;
use std::mem;
use std::time::Instant;
use smallvec::SmallVec;


#[derive(Debug)]
pub struct ChunkMesh {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    vertex_count: u32,
    index_count: u32,
}

impl ChunkMesh {
    pub async fn new(device: &Device, chunk: &Chunk) -> Self {
        log::debug!("Building mesh for chunk at: {}", chunk.position());
        let start_time = Instant::now();
        let mesh = Self::build_mesh(chunk);
        let indices = Self::get_indexes(&mesh);
        log::info!("Collected {} faces in {:?} for chunk at: {}", mesh.len(), Instant::now().duration_since(start_time), chunk.position());

        let vertex_buffer = {
            let buffer = device.create_buffer(&BufferDescriptor {
                label: Some(&format!("Chunk vertex buffer at: {}", chunk.position())),
                size: (mem::size_of::<TexturedArrayVertex>() * mesh.len()) as u64,
                usage: BufferUsages::VERTEX,
                mapped_at_creation: true
            });

            let slice = buffer.slice(..);
            slice.get_mapped_range_mut().copy_from_slice(bytemuck::cast_slice(&mesh));

            buffer.unmap();

            buffer
        };

        let index_buffer = {
            let buffer = device.create_buffer(&BufferDescriptor {
                label: Some(&format!("Chunk index buffer at: {}", chunk.position())),
                size: (mem::size_of::<u32>() * (mesh.len() / 4) * 6) as u64,
                usage: BufferUsages::INDEX,
                mapped_at_creation: true
            });

            let slice = buffer.slice(..);
            slice.get_mapped_range_mut().copy_from_slice(bytemuck::cast_slice(&indices));

            buffer.unmap();


            buffer
        };

        let vertex_count = mesh.len() as u32;
        let index_count = indices.len() as u32;

        Self {
            vertex_buffer,
            vertex_count,
            index_buffer,
            index_count
        }
    }

    fn get_indexes(vertices: &Vec<TexturedArrayVertex>) -> Vec<u32> {
        let mut indices = Vec::with_capacity(vertices.len());
        for n in 0..(vertices.len() / 4) {
            let offset = n*4;
            indices.push((0 + offset) as u32);
            indices.push((1 + offset) as u32);
            indices.push((2 + offset) as u32);
            indices.push((3 + offset) as u32);
            indices.push((2 + offset) as u32);
            indices.push((1 + offset) as u32);
        }

        indices
    }

    fn build_mesh(chunk: &Chunk) -> Vec<TexturedArrayVertex> {
        let mut all_faces = Vec::with_capacity(25_000);

        for index in 0..(CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) {
            let faces = Self::faces_for_block(index, &chunk);
            all_faces.extend(faces)
        }

        all_faces
    }

    fn faces_for_block(index: usize, chunk: &Chunk) -> SmallVec<[TexturedArrayVertex; 4 * 6]> {
        let mut faces = SmallVec::new();
        let original_offset = BlockOffset::from_index(index);
        let block_id = chunk.block(original_offset);
        if block_id != 0 {
            if let Some(offset) = original_offset.north() {
                let adjacent_block_id = chunk.block(offset);
                if adjacent_block_id == 0 {
                    faces.extend(Self::north_faces(original_offset))
                }
            } else {
                faces.extend(Self::north_faces(original_offset))
            }
            if let Some(offset) = original_offset.south() {
                let adjacent_block_id = chunk.block(offset);
                if adjacent_block_id == 0 {
                    faces.extend(Self::south_faces(original_offset))
                }
            } else {
                faces.extend(Self::south_faces(original_offset))
            }
            if let Some(offset) = original_offset.west() {
                let adjacent_block_id = chunk.block(offset);
                if adjacent_block_id == 0 {
                    faces.extend(Self::west_faces(original_offset))
                }
            } else {
                faces.extend(Self::west_faces(original_offset))
            }
            if let Some(offset) = original_offset.east() {
                let adjacent_block_id = chunk.block(offset);
                if adjacent_block_id == 0 {
                    faces.extend(Self::east_faces(original_offset))
                }
            } else {
                faces.extend(Self::east_faces(original_offset))
            }
            if let Some(offset) = original_offset.up() {
                let adjacent_block_id = chunk.block(offset);
                if adjacent_block_id == 0 {
                    faces.extend(Self::up_faces(original_offset))
                }
            } else {
                faces.extend(Self::up_faces(original_offset))
            }
            if let Some(offset) = original_offset.down() {
                let adjacent_block_id = chunk.block(offset);
                if adjacent_block_id == 0 {
                    faces.extend(Self::down_faces(original_offset))
                }
            } else {
                faces.extend(Self::down_faces(original_offset))
            }
        }

        faces
    }



    fn north_faces(offset: BlockOffset) -> [TexturedArrayVertex; 4] {
        let x = offset.x as f32;
        let y = offset.y as f32;
        let z = offset.z as f32;

        [
            TexturedArrayVertex {
                position: [x, y, z],
                tex_coords: [0.0, 0.0],
                tex_index: 0
            },
            TexturedArrayVertex {
                position: [x + 1.0, y, z],
                tex_coords: [1.0, 0.0],
                tex_index: 0
            },
            TexturedArrayVertex {
                position: [x, y + 1.0, z],
                tex_coords: [0.0, 1.0],
                tex_index: 0
            },
            TexturedArrayVertex {
                position: [x + 1.0, y + 1.0, z],
                tex_coords: [1.0, 1.0],
                tex_index: 0
            },
        ]
    }

    fn south_faces(offset: BlockOffset) -> [TexturedArrayVertex; 4] {
        let x = offset.x as f32;
        let y = offset.y as f32;
        let z = offset.z as f32;

        [
            TexturedArrayVertex {
                position: [x, y, z + 1.0],
                tex_coords: [0.0, 0.0],
                tex_index: 0
            },
            TexturedArrayVertex {
                position: [x + 1.0, y, z + 1.0],
                tex_coords: [1.0, 0.0],
                tex_index: 0
            },
            TexturedArrayVertex {
                position: [x, y + 1.0, z + 1.0],
                tex_coords: [0.0, 1.0],
                tex_index: 0
            },
            TexturedArrayVertex {
                position: [x + 1.0, y + 1.0, z + 1.0],
                tex_coords: [1.0, 1.0],
                tex_index: 0
            },
        ]
    }

    fn west_faces(offset: BlockOffset) -> [TexturedArrayVertex; 4] {
        // TODO
        let x = offset.x as f32;
        let y = offset.y as f32;
        let z = offset.z as f32;

        [
            TexturedArrayVertex {
                position: [x, y, z + 1.0],
                tex_coords: [1.0, 1.0],
                tex_index: 0
            },
            TexturedArrayVertex {
                position: [x + 1.0, y, z + 1.0],
                tex_coords: [1.0, 1.0],
                tex_index: 0
            },
            TexturedArrayVertex {
                position: [x, y + 1.0, z + 1.0],
                tex_coords: [1.0, 1.0],
                tex_index: 0
            },
            TexturedArrayVertex {
                position: [x + 1.0, y + 1.0, z + 1.0],
                tex_coords: [1.0, 1.0],
                tex_index: 0
            },
        ]
    }

    fn east_faces(offset: BlockOffset) -> [TexturedArrayVertex; 4] {
        // TODO
        let x = offset.x as f32;
        let y = offset.y as f32;
        let z = offset.z as f32;

        [
            TexturedArrayVertex {
                position: [x, y, z + 1.0],
                tex_coords: [1.0, 1.0],
                tex_index: 0
            },
            TexturedArrayVertex {
                position: [x + 1.0, y, z + 1.0],
                tex_coords: [1.0, 1.0],
                tex_index: 0
            },
            TexturedArrayVertex {
                position: [x, y + 1.0, z + 1.0],
                tex_coords: [1.0, 1.0],
                tex_index: 0
            },
            TexturedArrayVertex {
                position: [x + 1.0, y + 1.0, z + 1.0],
                tex_coords: [1.0, 1.0],
                tex_index: 0
            },
        ]
    }

    fn up_faces(offset: BlockOffset) -> [TexturedArrayVertex; 4] {
        // TODO
        let x = offset.x as f32;
        let y = offset.y as f32;
        let z = offset.z as f32;

        [
            TexturedArrayVertex {
                position: [x, y, z + 1.0],
                tex_coords: [1.0, 1.0],
                tex_index: 0
            },
            TexturedArrayVertex {
                position: [x + 1.0, y, z + 1.0],
                tex_coords: [1.0, 1.0],
                tex_index: 0
            },
            TexturedArrayVertex {
                position: [x, y + 1.0, z + 1.0],
                tex_coords: [1.0, 1.0],
                tex_index: 0
            },
            TexturedArrayVertex {
                position: [x + 1.0, y + 1.0, z + 1.0],
                tex_coords: [1.0, 1.0],
                tex_index: 0
            },
        ]
    }

    fn down_faces(offset: BlockOffset) -> [TexturedArrayVertex; 4] {
        // TODO
        let x = offset.x as f32;
        let y = offset.y as f32;
        let z = offset.z as f32;

        [
            TexturedArrayVertex {
                position: [x, y, z + 1.0],
                tex_coords: [1.0, 1.0],
                tex_index: 0
            },
            TexturedArrayVertex {
                position: [x + 1.0, y, z + 1.0],
                tex_coords: [1.0, 1.0],
                tex_index: 0
            },
            TexturedArrayVertex {
                position: [x, y + 1.0, z + 1.0],
                tex_coords: [1.0, 1.0],
                tex_index: 0
            },
            TexturedArrayVertex {
                position: [x + 1.0, y + 1.0, z + 1.0],
                tex_coords: [1.0, 1.0],
                tex_index: 0
            },
        ]
    }

    pub fn render<'a>(&'a mut self, render_context: &RenderContext, render_pass: &mut RenderPass<'a>) {
        render_pass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint32);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw_indexed(0..self.index_count, 0, 0..1);
    }
}
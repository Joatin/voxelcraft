use crate::game::resources::construct_block_pipeline::construct_block_pipeline;
use camera::wgpu::WgpuFpsCamera;
use cgmath::Deg;
use face_texture_map::{FaceTextureMap, FaceTextureMapBuilder};
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use voxelcraft_id::BlockId;
use voxelcraft_mod::{Block, ModPack};
use wgpu::{Device, Queue, RenderPipeline, TextureFormat};
use wgpu_async_utils::geometry_buffer::StandardGeometryBuffer;

pub struct GameResources {
    pub geometry_buffer: StandardGeometryBuffer,
    pub face_texture_map: Arc<FaceTextureMap>,
    pub block_pipeline: RenderPipeline,
    pub camera: WgpuFpsCamera,
    pub blocks: Arc<HashMap<BlockId, Arc<dyn Block>>>,
}

impl GameResources {
    pub async fn new(
        queue: &Queue,
        device: &Device,
        texture_format: TextureFormat,
        width: u32,
        height: u32,
        mod_pack: &Arc<dyn ModPack>,
    ) -> Result<Self, Box<dyn Error + Send + Sync>> {
        log::info!("Allocating g-buffer resources");
        let geometry_buffer =
            StandardGeometryBuffer::new(device, width, height, "game geometry buffer").await;

        let camera = WgpuFpsCamera::new(
            device,
            (0.0, 2.0, 0.0),
            Deg(-90.0),
            Deg(-20.0),
            width as f32,
            height as f32,
        )
        .await;

        let (face_texture_map, blocks) = {
            let mut face_texture_map_builder = FaceTextureMapBuilder::default();
            let mut blocks = HashMap::new();

            for module in mod_pack.mods() {
                for block in module.register_blocks().await {
                    for (face_id, mipmap) in block.register_faces() {
                        log::info!(
                            "Adding face '{:?}' for block '{}' in module '{}'",
                            face_id,
                            block.name(),
                            module.name()
                        );
                        face_texture_map_builder.with_image((*face_id).clone(), mipmap);
                    }
                    blocks.insert(*block.block_id(), block);
                }
            }

            (face_texture_map_builder.build(queue, device).await?, blocks)
        };

        let face_texture_map = Arc::new(face_texture_map);
        let blocks = Arc::new(blocks);

        log::info!("Registered a total of {} blocks", blocks.len());

        let block_pipeline = construct_block_pipeline(
            device,
            texture_format,
            face_texture_map.bind_group_layout(),
            camera.bind_group_layout(),
        )
        .await;

        Ok(Self {
            geometry_buffer,
            face_texture_map,
            camera,
            block_pipeline,
            blocks,
        })
    }

    pub async fn resize(&mut self, device: &Device, width: u32, height: u32) {
        self.geometry_buffer =
            StandardGeometryBuffer::new(device, width, height, "game geometry buffer").await;
        self.camera.set_aspect_ratio(width as f32, height as f32);
    }

    pub fn cleanup(&mut self) {
        self.camera.cleanup();
    }
}

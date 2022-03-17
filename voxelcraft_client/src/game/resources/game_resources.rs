use crate::game::resources::construct_block_pipeline::construct_block_pipeline;
use camera::wgpu::WgpuFpsCamera;
use cgmath::Deg;
use face_texture_map::{FaceTextureMap, FaceTextureMapBuilder};
use std::error::Error;
use wgpu::{Device, Queue, RenderPipeline, TextureFormat};
use wgpu_async_utils::geometry_buffer::StandardGeometryBuffer;

pub struct GameResources {
    pub geometry_buffer: StandardGeometryBuffer,
    pub face_texture_map: FaceTextureMap,
    pub block_pipeline: RenderPipeline,
    pub camera: WgpuFpsCamera,
}

impl GameResources {
    pub async fn new(
        queue: &Queue,
        device: &Device,
        texture_format: TextureFormat,
        width: u32,
        height: u32,
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

        let face_texture_map = FaceTextureMapBuilder::default()
            .build(queue, device)
            .await?;

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

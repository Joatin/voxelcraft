use crate::texture::DepthTexture;
use std::sync::Arc;
use wgpu::{Device, TextureFormat, TextureView};

pub struct StandardGeometryBuffer {
    depth_texture: DepthTexture,
}

impl StandardGeometryBuffer {
    pub async fn new(device: &Device, width: u32, height: u32, label: &str) -> Self {
        let depth_texture = DepthTexture::new(
            device,
            TextureFormat::Depth32Float,
            width,
            height,
            &format!("{} depth texture", label),
        )
        .await;

        Self { depth_texture }
    }

    pub fn depth_view(&self) -> &TextureView {
        &self.depth_texture.view
    }
}

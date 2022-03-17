use std::error::Error;
use std::sync::Arc;
use wgpu::{Device, SurfaceConfiguration, Texture, TextureFormat, TextureView};
use wgpu_tokio::{DeviceAsyncExt, TextureAsyncExt};

pub struct DepthTexture {
    pub texture: Texture,
    pub view: TextureView,
    pub format: TextureFormat,
}

impl DepthTexture {
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    pub async fn new(
        device: &Device,
        format: TextureFormat,
        width: u32,
        height: u32,
        label: &str,
    ) -> Self {
        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let desc = wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        };
        let texture = device.create_texture_async(&desc).await;

        let view = texture
            .create_view_async(&wgpu::TextureViewDescriptor::default())
            .await;

        Self {
            texture,
            view,
            format,
        }
    }
}

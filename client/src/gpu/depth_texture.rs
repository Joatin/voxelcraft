use crate::primitives::Size;
use std::error::Error;
use std::sync::Arc;
use wgpu::{Device, Texture, TextureView};

pub struct DepthTexture {
    texture: Texture,
    view: TextureView,
}

impl DepthTexture {
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    pub async fn new(
        device: &Arc<Device>,
        size: Size,
        label: &str,
    ) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let device = Arc::clone(&device);
        let label = label.to_string();
        let handle = tokio::task::spawn_blocking(move || {
            let size = wgpu::Extent3d {
                width: size.width as u32,
                height: size.height as u32,
                depth_or_array_layers: 1,
            };

            let desc = wgpu::TextureDescriptor {
                label: Some(&label),
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: Self::DEPTH_FORMAT,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING,
            };
            let texture = device.create_texture(&desc);

            let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

            (texture, view)
        });

        let (texture, view) = handle.await?;

        Ok(Self { texture, view })
    }
}

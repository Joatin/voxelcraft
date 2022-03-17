use std::num::NonZeroU32;
use std::sync::Arc;
use wgpu::{
    Device, Extent3d, Queue, SamplerDescriptor, Texture, TextureFormat, TextureUsages, TextureView,
    TextureViewDescriptor, TextureViewDimension,
};
use wgpu_tokio::{DeviceAsyncExt, TextureAsyncExt};

#[derive(Debug)]
pub struct TextureArray {
    pub texture: Texture,
    pub view: TextureView,
}

impl TextureArray {
    pub async fn new_from_compressed(
        queue: &Queue,
        device: &Device,
        format: TextureFormat,
        mip_level_count: u32,
        sample_count: u32,
        width: u32,
        height: u32,
        images: &Vec<Vec<u8>>,
        label: &str,
    ) -> Self {
        let mut processed_images = vec![];
        for data in images {
            let diffuse_image = image::load_from_memory(data).unwrap();
            let diffuse_rgba = diffuse_image.into_rgba8();
            processed_images.push(diffuse_rgba.into_raw())
        }
        Self::new(
            queue,
            device,
            format,
            mip_level_count,
            sample_count,
            width,
            height,
            &processed_images,
            label,
        )
        .await
    }

    pub async fn new(
        queue: &Queue,
        device: &Device,
        format: TextureFormat,
        mip_level_count: u32,
        sample_count: u32,
        width: u32,
        height: u32,
        images: &Vec<Vec<u8>>,
        label: &str,
    ) -> Self {
        let array_layer_count = images.len() as u32;

        let texture_size = Extent3d {
            width,
            height,
            depth_or_array_layers: array_layer_count,
        };

        let flat_image = images.iter().cloned().flatten().collect::<Vec<_>>();

        let texture = device
            .create_texture_with_data_async(
                queue,
                &wgpu::TextureDescriptor {
                    size: texture_size,
                    mip_level_count,
                    sample_count,
                    dimension: wgpu::TextureDimension::D2,
                    format,
                    usage: TextureUsages::TEXTURE_BINDING,
                    label: Some(label),
                },
                &flat_image,
            )
            .await;

        let view = texture
            .create_view_async(&TextureViewDescriptor {
                label: Some(label),
                format: Some(format),
                dimension: Some(TextureViewDimension::D2Array),
                aspect: Default::default(),
                base_mip_level: 0,
                mip_level_count: NonZeroU32::new(mip_level_count),
                base_array_layer: 0,
                array_layer_count: Some(NonZeroU32::new(array_layer_count).unwrap()),
            })
            .await;

        Self { texture, view }
    }
}

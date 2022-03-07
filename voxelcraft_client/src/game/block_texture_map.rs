use std::error::Error;
use wgpu::{Extent3d, Device, Texture, Queue, BindGroup, BindGroupLayout};
use std::sync::Arc;

#[derive(Debug)]
pub struct BlockTextureMap {
    standard_texture: Texture,
    bind_group: BindGroup,
    bind_group_layout: BindGroupLayout
}

impl BlockTextureMap {
    pub async fn new(device: &Arc<Device>, queue: &Arc<Queue>) -> Result<Self, Box<dyn Error>> {
        let texture_size = Extent3d {
            width: 32,
            height: 32,
            depth_or_array_layers: 2,
        };

        let standard_texture = device.create_texture(
            &wgpu::TextureDescriptor {
                size: texture_size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                label: Some("Standard Block Textures"),
            }
        );

        // queue.write_texture(
        //     wgpu::ImageCopyTexture {
        //         aspect: wgpu::TextureAspect::All,
        //         texture: &standard_texture,
        //         mip_level: 0,
        //         origin: wgpu::Origin3d::ZERO,
        //     },
        //     rgba,
        //     wgpu::ImageDataLayout {
        //         offset: 0,
        //         bytes_per_row: std::num::NonZeroU32::new(4 * 32),
        //         rows_per_image: std::num::NonZeroU32::new(32),
        //     },
        //     size,
        // );

        let view = standard_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(
            &wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::Repeat,
                address_mode_v: wgpu::AddressMode::Repeat,
                address_mode_w: wgpu::AddressMode::Repeat,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            }
        );

        let bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2Array,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(
                            wgpu::SamplerBindingType::Filtering,
                        ),
                        count: None,
                    },
                ],
                label: Some("Standard Block Bind Group Layout"),
            }
        );

        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    }
                ],
                label: Some("Standard Block Bind Group"),
            }
        );


        Ok(Self {
            standard_texture,
            bind_group,
            bind_group_layout
        })
    }

    pub fn get_texture_index_for_face(&self, block_id: u32) -> i32 {
        0
    }

    pub fn texture(&self) -> &Texture {
        &self.standard_texture
    }

    pub fn bind_group(&self) -> &BindGroup {
        &self.bind_group
    }

    pub fn bind_group_layout(&self) -> &BindGroupLayout {
        &self.bind_group_layout
    }

}
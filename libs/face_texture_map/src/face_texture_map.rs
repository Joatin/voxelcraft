use mipmap::Mipmap;
use std::collections::HashMap;
use std::error::Error;
use std::num::NonZeroU8;
use std::sync::Arc;
use voxelcraft_mod::{FaceId, ModId};
use wgpu::{BindGroup, BindGroupLayout, Device, Extent3d, Queue, Sampler, Texture, TextureFormat};
use wgpu_async_utils::texture::TextureArray;
use wgpu_tokio::DeviceAsyncExt;

#[derive(Debug)]
pub struct FaceTextureMap {
    texture: TextureArray,
    sampler: Sampler,
    bind_group: BindGroup,
    bind_group_layout: BindGroupLayout,
    mappings: HashMap<(ModId, FaceId), i32>,
}

const MIPMAPS: Mipmap = mipmap::include_mips!("rainbow_block.png");

impl FaceTextureMap {
    pub(crate) async fn new(
        queue: &Queue,
        device: &Device,
        images: &mut Vec<((ModId, FaceId), Vec<u8>)>,
    ) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let mips = mipmap::include_mips!("rainbow_block.png");
        let default_image = MIPMAPS.get_all();

        let (mappings, images) = {
            let mut mappings = HashMap::new();

            let mut new_images = vec![default_image];

            for (index, (key, data)) in images.drain(..).enumerate() {
                new_images.push(data);
                mappings.insert(key, (index + 1) as i32);
            }

            (mappings, new_images)
        };

        let texture = TextureArray::new(
            queue,
            device,
            TextureFormat::Rgba8UnormSrgb,
            MIPMAPS.num_layers() as u32,
            1,
            32,
            32,
            &images,
            "Face Texture Map 3D Texture",
        )
        .await;

        let sampler = device
            .create_sampler_async(&wgpu::SamplerDescriptor {
                label: None,
                address_mode_u: wgpu::AddressMode::Repeat,
                address_mode_v: wgpu::AddressMode::Repeat,
                address_mode_w: wgpu::AddressMode::Repeat,
                mag_filter: wgpu::FilterMode::Nearest,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                lod_min_clamp: 6.0,
                lod_max_clamp: 0.0,
                compare: None,
                anisotropy_clamp: Some(NonZeroU8::new(16).unwrap()),
                border_color: None,
            })
            .await;

        let bind_group_layout = device
            .create_bind_group_layout_async(&wgpu::BindGroupLayoutDescriptor {
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
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("Standard Block Bind Group Layout"),
            })
            .await;

        let bind_group = device
            .create_bind_group_async(&wgpu::BindGroupDescriptor {
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    },
                ],
                label: Some("Standard Block Bind Group"),
            })
            .await;

        Ok(Self {
            texture,
            sampler,
            bind_group,
            bind_group_layout,
            mappings,
        })
    }

    pub fn get_texture_index_for_face(&self, mod_id: ModId, face_id: FaceId) -> i32 {
        self.mappings.get(&(mod_id, face_id)).map_or(0, |i| *i)
    }

    pub fn texture(&self) -> &Texture {
        &self.texture.texture
    }

    pub fn sampler(&self) -> &Sampler {
        &self.sampler
    }

    pub fn bind_group(&self) -> &BindGroup {
        &self.bind_group
    }

    pub fn bind_group_layout(&self) -> &BindGroupLayout {
        &self.bind_group_layout
    }
}

use std::collections::HashMap;
use wgpu::{BindGroup, BindGroupLayout};
use crate::gpu::Gpu;
use crate::gpu::Texture;

pub struct TextureMap {
    textures: HashMap<&'static str, (Texture, BindGroup)>

}

impl TextureMap {
    pub fn new() -> Self {


        let textures = HashMap::new();

        Self {
            textures
        }
    }

    pub fn insert_default_textures(&mut self, state: &Gpu, texture_bind_group_layout: &BindGroupLayout) {
        self.textures.insert("default:button", Self::create_texture_and_bind_group_from_bytes("default:button", include_bytes!("textures/button.png"), state, texture_bind_group_layout));
    }

    pub fn get(&self, key: &str) -> Option<&(Texture, BindGroup)> {
        self.textures.get(key)
    }

fn create_texture_and_bind_group_from_bytes(label: &str, image: &[u8], state: &Gpu, texture_bind_group_layout: &BindGroupLayout) -> (Texture, BindGroup) {
        let texture = Texture::from_bytes(state, image, label).unwrap();
        let diffuse_bind_group = state.device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture.view), // CHANGED!
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&texture.sampler), // CHANGED!
                    }
                ],
                label: Some("diffuse_bind_group"),
            }
        );

        (texture, diffuse_bind_group)
    }
}
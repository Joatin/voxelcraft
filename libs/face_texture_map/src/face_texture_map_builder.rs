use crate::face_texture_map::FaceTextureMap;
use std::error::Error;
use std::sync::Arc;
use voxelcraft_mod::{FaceId, ModId};
use wgpu::{Device, Queue};

#[derive(Default, Debug, Clone)]
pub struct FaceTextureMapBuilder {
    images: Vec<((ModId, FaceId), Vec<u8>)>,
}

impl FaceTextureMapBuilder {
    pub fn with_image(&mut self, mod_id: ModId, face_id: FaceId, image: &[u8]) -> &mut Self {
        self.images.push(((mod_id, face_id), image.to_vec()));
        self
    }

    pub async fn build(
        &mut self,
        queue: &Queue,
        device: &Device,
    ) -> Result<FaceTextureMap, Box<dyn Error + Send + Sync>> {
        FaceTextureMap::new(queue, device, &mut self.images).await
    }
}

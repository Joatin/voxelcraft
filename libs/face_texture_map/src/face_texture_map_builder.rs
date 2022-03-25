use crate::face_texture_map::FaceTextureMap;
use mipmap::Mipmap;
use std::error::Error;
use std::sync::Arc;
use voxelcraft_id::{FaceId, ModId};
use wgpu::{Device, Queue};

#[derive(Default, Debug, Clone)]
pub struct FaceTextureMapBuilder {
    images: Vec<(FaceId, &'static Mipmap<'static>)>,
}

impl FaceTextureMapBuilder {
    pub fn with_image(&mut self, face_id: FaceId, image: &'static Mipmap<'static>) -> &mut Self {
        self.images.push((face_id, image));
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

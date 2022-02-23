use std::sync::Arc;
use wgpu::{Device, TextureView};
use winit::dpi::PhysicalSize;

#[derive(Clone)]
pub struct RenderContext {
    pub device: Arc<Device>,
    pub view: Arc<TextureView>,
    pub size: PhysicalSize<u32>
}

impl RenderContext {
    pub fn new(device: &Arc<Device>, view: &Arc<TextureView>, size: &PhysicalSize<u32>) -> Self {
        Self {
            device: Arc::clone(device),
            view: Arc::clone(view),
            size: size.to_owned()
        }
    }
}
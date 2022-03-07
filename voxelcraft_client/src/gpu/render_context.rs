use crate::primitives::{Point2D, Size};

use std::sync::{Arc};
use wgpu::{Device, TextureView};
use winit::dpi::PhysicalSize;

#[derive(Clone)]
pub struct RenderContext {
    pub device: Arc<Device>,
    pub view: Arc<TextureView>,
    pub size: Size,
    pub cursor_position: Point2D,
}

impl RenderContext {
    pub fn new(device: &Arc<Device>, view: &Arc<TextureView>, size: &PhysicalSize<u32>) -> Self {
        Self {
            device: Arc::clone(device),
            view: Arc::clone(view),
            size: size.clone().into(),
            cursor_position: Point2D::default(),
        }
    }
}

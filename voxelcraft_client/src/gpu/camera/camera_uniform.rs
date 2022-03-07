use crate::gpu::camera::{Camera, Projection};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
    view_position: [f32; 4],
}

impl CameraUniform {
    pub fn new(camera: &Camera, projection: &Projection) -> Self {
        Self {
            view_position: camera.position.to_homogeneous().into(),
            view_proj: (projection.calc_matrix() * camera.calc_matrix()).into(),
        }
    }
}

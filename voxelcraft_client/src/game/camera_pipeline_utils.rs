use wgpu::{BindGroupLayout, BindGroup, Buffer, Device, BufferDescriptor, MapMode, CommandEncoder};
use std::mem;
use crate::gpu::camera::Camera;
use crate::gpu::camera::Projection;
use crate::gpu::camera::CameraUniform;
use wgpu::util::StagingBelt;
use std::num::NonZeroU64;
use tokio::sync::Mutex;
use std::sync::Arc;
use std::fmt::{Debug, Formatter};

pub struct CameraPipelineUtils {
    bind_group_layout: BindGroupLayout,
    bind_group: BindGroup,
    camera_buffer: Buffer,
    staging_belt: Arc<Mutex<StagingBelt>>
}

impl Debug for CameraPipelineUtils {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl CameraPipelineUtils {
    pub fn new(device: &Device) -> Self {


        let camera_buffer = device.create_buffer(
            &BufferDescriptor {
                label: Some("Camera Buffer"),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                size: mem::size_of::<CameraUniform>() as u64,
                mapped_at_creation: false
            }
        );

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("camera_bind_group_layout"),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                }
            ],
            label: Some("camera_bind_group"),
        });

        Self {
            bind_group_layout,
            bind_group,
            camera_buffer,
            staging_belt: Arc::new(Mutex::new(StagingBelt::new(1024)))
        }
    }

    pub fn bind_group(&self) -> &BindGroup {
        &self.bind_group
    }

    pub fn bind_group_layout(&self) -> &BindGroupLayout {
        &self.bind_group_layout
    }

    pub async fn update(&self, device: &Device, encoder: &mut CommandEncoder, camera: &Camera, projection: &Projection) {
        let mut lock = self.staging_belt.lock().await;
        {
            let mut slice = lock.write_buffer(
                encoder,
                &self.camera_buffer,
                0,
                NonZeroU64::new(mem::size_of::<CameraUniform>() as u64).unwrap(),
                device
            );

            slice.copy_from_slice(bytemuck::cast_slice(&[CameraUniform::new(camera, projection)]));
        }


        lock.finish()
    }

    pub fn cleanup(&self) {
        let staging_belt = Arc::clone(&self.staging_belt);
        tokio::spawn(async move {
            let fut = {
                let mut lock = staging_belt.lock().await;
                lock.recall()
            };
            fut.await
        });
    }
}
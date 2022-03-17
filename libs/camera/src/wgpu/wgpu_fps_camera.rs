use crate::wgpu::camera_uniform::CameraUniform;
use crate::FpsCamera;
use cgmath::{Deg, Point3};
use std::fmt::{Debug, Formatter};
use std::mem;
use std::num::NonZeroU64;
use std::ops::{Deref, DerefMut};
use wgpu::util::StagingBelt;
use wgpu::{BindGroup, BindGroupLayout, Buffer, BufferDescriptor, CommandEncoder, Device};
use wgpu_tokio::DeviceAsyncExt;

pub struct WgpuFpsCamera {
    camera: FpsCamera,
    bind_group_layout: BindGroupLayout,
    bind_group: BindGroup,
    camera_buffer: Buffer,
    staging_belt: StagingBelt,
}

impl WgpuFpsCamera {
    pub async fn new<V: Into<Point3<f32>>>(
        device: &Device,
        position: V,
        yaw: Deg<f32>,
        pitch: Deg<f32>,
        width: f32,
        height: f32,
    ) -> Self {
        let camera = FpsCamera::new(position, yaw, pitch, width, height);

        let camera_buffer = device
            .create_buffer_async(&BufferDescriptor {
                label: Some("Camera Buffer"),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                size: mem::size_of::<CameraUniform>() as u64,
                mapped_at_creation: false,
            })
            .await;

        let bind_group_layout = device
            .create_bind_group_layout_async(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            })
            .await;

        let bind_group = device
            .create_bind_group_async(&wgpu::BindGroupDescriptor {
                layout: &bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                }],
                label: Some("camera_bind_group"),
            })
            .await;

        let staging_belt = StagingBelt::new(1024);

        Self {
            camera,
            camera_buffer,
            bind_group_layout,
            bind_group,
            staging_belt,
        }
    }

    pub fn bind_group(&self) -> &BindGroup {
        &self.bind_group
    }

    pub fn bind_group_layout(&self) -> &BindGroupLayout {
        &self.bind_group_layout
    }

    pub fn write_camera(&mut self, device: &Device, encoder: &mut CommandEncoder) {
        let new_uniform = CameraUniform {
            view_proj: self.calc_matrix().into(),
        };

        {
            let mut slice = self.staging_belt.write_buffer(
                encoder,
                &self.camera_buffer,
                0,
                NonZeroU64::new(mem::size_of::<CameraUniform>() as u64).unwrap(),
                device,
            );

            slice.copy_from_slice(bytemuck::cast_slice(&[new_uniform]));
        }

        self.staging_belt.finish()
    }

    pub fn cleanup(&mut self) {
        tokio::spawn(self.staging_belt.recall());
    }
}

impl Debug for WgpuFpsCamera {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Deref for WgpuFpsCamera {
    type Target = FpsCamera;

    fn deref(&self) -> &Self::Target {
        &self.camera
    }
}

impl DerefMut for WgpuFpsCamera {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.camera
    }
}

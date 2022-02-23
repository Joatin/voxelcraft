use crate::interface::widget::Widget;
use crate::gpu::{RenderContext, Gpu};
use wgpu::{CommandEncoder, RenderPassDescriptor, Buffer};
use wgpu_glyph::GlyphBrush;
use image::GenericImageView;
use crate::gpu::Texture;
use crate::interface::texture_map::TextureMap;
use crate::gpu::primitives::TexturedVertex;
use wgpu::util::{DeviceExt, StagingBelt};
use crate::interface::screen_context::ScreenContext;
use std::mem;
use std::num::NonZeroU64;
use winit::dpi::PhysicalSize;


pub struct ImageWidget {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    staging_belt: StagingBelt
}

const INDICES: &[u16] = &[
    2, 1, 0,
    1, 2, 3,
];


impl ImageWidget {

    pub fn new(state: &Gpu, texture: &str, x: f32, y: f32, width: f32, height: f32) -> Self {

        let vertices = Self::build_vertices(x, y, width, height);
        let vertex_buffer = state.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        let index_buffer = state.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        let staging_belt = StagingBelt::new(512);


        Self {
            x,
            y,
            width,
            height,
            vertex_buffer,
            index_buffer,
            staging_belt
        }
    }

    fn build_vertices(x: f32, y: f32, width: f32, height: f32) -> Vec<TexturedVertex> {
        log::warn!("{} {} {} {}", x, y, width, height);
        vec![
            TexturedVertex { position: [x, y - height, 0.0], tex_coords: [0.0, 0.0] },
            TexturedVertex { position: [x, y, 0.0], tex_coords: [0.0, 1.0] },
            TexturedVertex { position: [x + width, y - height, 0.0], tex_coords: [1.0, 0.0] },
            TexturedVertex { position: [x + width, y, 0.0], tex_coords: [1.0, 1.0] },
        ]
    }
}

impl Widget for ImageWidget {
    fn widgets(&mut self) -> Vec<&mut dyn Widget> {
        vec![]
    }

    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }

    fn render(&mut self, render_context: &RenderContext, screen_context: &mut ScreenContext, offset: (f32, f32)) {
        {
            let mut view = self.staging_belt.write_buffer(
                screen_context.encoder,
                &self.vertex_buffer,
                0,
                NonZeroU64::new((mem::size_of::<TexturedVertex>() * 4) as u64).unwrap(),
                &render_context.device
            );

            let vertices = Self::build_vertices(get_x_transformed_coord(self.x + offset.0, &render_context.size), get_y_transformed_coord(self.y + offset.1, &render_context.size), get_x_transformed_width(self.width, &render_context.size), get_y_transformed_height(self.height, &render_context.size));
            view.copy_from_slice( bytemuck::cast_slice(&vertices));
        }


        self.staging_belt.finish();

        let mut render_pass = screen_context.encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("default_button"),
            color_attachments: &[
                wgpu::RenderPassColorAttachment {
                    view: &render_context.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                }
            ],
            depth_stencil_attachment: None
        });

        let (render_pipeline, _) = screen_context.pipeline_map.get("default:pipeline").expect("This pipeline should always exist");
        let (_texture, bind_group) = screen_context.texture_map.get("default:button").expect("This texture should always exist");

        render_pass.set_pipeline(render_pipeline);
        render_pass.set_bind_group(0, &bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        render_pass.draw_indexed(0..INDICES.len() as u32, 0, 0..1);

    }

    fn cleanup(&mut self) {
        tokio::spawn(self.staging_belt.recall());
    }
}


fn get_x_transformed_coord(pos_x: f32, size: &PhysicalSize<u32>) -> f32 {
    pos_x * 2.0 / size.width as f32 - 1.0
}

fn get_y_transformed_coord(pos_y: f32, size: &PhysicalSize<u32>) -> f32 {
    (pos_y * 2.0 / size.height as f32 - 1.0) * -1.0
}

fn get_x_transformed_width(width: f32, size: &PhysicalSize<u32>) -> f32 {
    2.0 / size.width as f32 * width
}

fn get_y_transformed_height(height: f32, size: &PhysicalSize<u32>) -> f32 {
    2.0 / size.height as f32 * height
}
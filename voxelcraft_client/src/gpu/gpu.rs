use winit::window::Window;
use winit::event::WindowEvent;
use winit::dpi::PhysicalSize;
use wgpu::{TextureView, TextureFormat, CommandBuffer};
use futures::Future;
use std::sync::Arc;
use pollster::FutureExt;
use crate::gpu::RenderContext;
use std::time::Instant;
use crate::context::Context;

pub struct Gpu {
    surface: wgpu::Surface,
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
    config: wgpu::SurfaceConfiguration,
    pub size: PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    pub render_format: TextureFormat,
    last_render: Instant
}

impl Gpu {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: &Window) -> Self {
        log::info!("Creating new GPU instance");
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None, // Trace path
        ).await.unwrap();

        let shader = device.create_shader_module(&wgpu::include_wgsl!("shader.wgsl"));

        let render_format = surface.get_preferred_format(&adapter).unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: render_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        surface.configure(&device, &config);

        Self {
            surface,
            device: Arc::new(device),
            queue: Arc::new(queue),
            config,
            size,
            render_pipeline,
            render_format,
            last_render: Instant::now()
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        log::info!("Resize: {:?}", new_size);
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn start_render_pass<'a, T: FnOnce(RenderContext) -> Vec<CommandBuffer>>(&'a mut self, mut render_callback: T) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;

        let start_draw_time = Instant::now();

        let view = Arc::new(output.texture.create_view(&wgpu::TextureViewDescriptor::default()));

        let render_context = RenderContext::new(&self.device, &view, &self.size);
        let command_buffers = render_callback(render_context);

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.4,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.draw(0..3, 0..1);
        }

        self.queue.submit(vec![encoder.finish()]);
        self.queue.submit(command_buffers);


        // {
        //     let time_to_render = Instant::now().duration_since(start_draw_time);
        //     self.context.set_time_to_draw_frame(time_to_render);
        // }

        output.present();

        // {
        //     let finish_time = Instant::now();
        //     let duration_between_last_frame = finish_time.duration_since(self.last_render);
        //     self.context.set_current_fps_from_duration(duration_between_last_frame);
        //     self.last_render = finish_time;
        // }


        Ok(())

    }
}
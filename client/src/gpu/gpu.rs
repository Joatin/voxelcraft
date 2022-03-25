use crate::gpu::RenderContext;

use std::sync::Arc;
use std::time::Instant;
use wgpu::{CommandBuffer, TextureFormat};
use winit::dpi::PhysicalSize;

use winit::window::Window;

pub struct Gpu {
    surface: wgpu::Surface,
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
    config: wgpu::SurfaceConfiguration,
    pub size: PhysicalSize<u32>,
    pub render_format: TextureFormat,
    last_render: Instant,
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

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None, // Trace path
            )
            .await
            .unwrap();

        let shader = device.create_shader_module(&wgpu::include_wgsl!("shader.wgsl"));

        let render_format = surface.get_preferred_format(&adapter).unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: render_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        surface.configure(&device, &config);

        Self {
            surface,
            device: Arc::new(device),
            queue: Arc::new(queue),
            config,
            size,
            render_format,
            last_render: Instant::now(),
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

    pub fn start_render_pass<'a, T: FnOnce(RenderContext) -> Vec<CommandBuffer>>(
        &'a mut self,
        render_callback: T,
    ) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;

        let _start_draw_time = Instant::now();

        let view = Arc::new(
            output
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default()),
        );

        let render_context = RenderContext::new(&self.device, &view, &self.size);
        let mut command_buffers = render_callback(render_context);

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

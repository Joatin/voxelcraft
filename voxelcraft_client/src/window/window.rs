use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use crate::gpu::Gpu;
use std::error::Error;
use crate::context::Context;
use crate::interface::{Screen, Page, HOME_ROUTE};
use tokio::task;
use winit::event_loop::EventLoopWindowTarget;
use winit::window::WindowId;
use pollster::FutureExt;
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::interface::widget::{TextWidget, ButtonWidget};

pub struct Window {
    event_loop: EventLoop<()>,
    window: winit::window::Window,
    gpu: Gpu,
    screen: Screen
}

impl Window {
    pub async fn new(context: &Arc<Context>) -> Result<Self, Box<dyn Error>> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().with_title("Voxelcraft").build(&event_loop)?;
        let gpu = Gpu::new(&context, &window).await;
        let screen = Screen::new(&context, &gpu, Screen::default_pages(&gpu));


        Ok(Self {
            event_loop,
            window,
            gpu,
            screen
        })
    }

    pub async fn run(mut self, context: Arc<Context>) {
        let event_loop = self.event_loop;
        let window = self.window;
        let mut gpu = self.gpu;
        let mut screen = self.screen;

        task::block_in_place(move || {
            // Loop forever
            event_loop.run(move |event, target, mut control_flow| {
                match event {
                    Event::WindowEvent {
                        event,
                        window_id,
                    } if window_id == window.id() =>  {
                        Self::on_window_event(&mut control_flow, &context, &mut gpu, &event);
                    },
                    Event::RedrawRequested(window_id)if window_id == window.id() => {
                        Self::on_redraw_requested(&mut control_flow, &context, &mut gpu, &mut screen);
                    },
                    Event::MainEventsCleared => {
                        // RedrawRequested will only trigger once, unless we manually
                        // request it.
                        window.request_redraw();
                    }
                    _ => {}
                }
            });
        });


    }

    fn on_window_event(control_flow: &mut ControlFlow, context: &Arc<Context>, gpu: &mut Gpu, event: &WindowEvent<'_>) {
        if !gpu.input(&event) {
            match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                },
                WindowEvent::KeyboardInput {
                    input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::F3),
                        ..
                    },
                    ..
                } => {
                    context.toggle_debug();
                }
                WindowEvent::Resized(physical_size) => {

                    gpu.resize(*physical_size);
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    // new_inner_size is &&mut so we have to dereference it twice
                    gpu.resize(**new_inner_size);
                }

                _ => {}
            }
        }
    }

    fn on_redraw_requested(control_flow: &mut ControlFlow, context: &Arc<Context>, gpu: &mut Gpu, screen: &mut Screen) {
        let res = match gpu.start_render_pass(|render_context| {
            if let Some(game) = context.get_game() {
                game.render(context, &render_context)
            }
            let command_buffer = screen.render(context, &render_context);

            vec![command_buffer]
        }) {
            Ok(_) => {},
            // Reconfigure the surface if lost
            Err(wgpu::SurfaceError::Lost) => {
                gpu.resize(gpu.size);
            },
            // The system is out of memory, we should probably quit
            Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
            // All other errors (Outdated, Timeout) should be resolved by the next frame
            Err(e) => {
                eprintln!("{:?}", e);
            },
        };

        screen.cleanup();

        // screen2.lock().await.cleanup();

        res
    }
}
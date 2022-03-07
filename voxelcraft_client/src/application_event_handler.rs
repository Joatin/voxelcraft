use crate::game::GameManager;
use crate::gpu::Gpu;
use crate::interface::Interface;
use crate::primitives::{Point2D, Size};
use crate::window::EventHandler;
use iced::mouse::Interaction;
use iced_native::Event;

use winit::event::ElementState;

pub struct ApplicationEventHandler {
    gpu: Gpu,
    interface: Interface,
    game_manager: GameManager,
}

impl ApplicationEventHandler {
    pub fn new(gpu: Gpu, interface: Interface, game_manager: GameManager) -> Self {
        Self {
            gpu,
            interface,
            game_manager,
        }
    }
}

impl EventHandler for ApplicationEventHandler {
    fn on_redraw_requested(&mut self) -> (bool, Interaction) {
        let mut continue_render = true;
        let cursor = Interaction::Pointer;

        match self.gpu.start_render_pass(|render_context| {
            let mut buffers = self.game_manager.render(&render_context);
            let game_messages = self.game_manager.get_messages();
            let (buffer, _cursor, should_quit) =
                self.interface.render(&render_context, game_messages, |m| {
                    self.game_manager.process_message(m)
                });
            buffers.push(buffer);
            continue_render = !should_quit;
            buffers
        }) {
            Err(wgpu::SurfaceError::Lost) => {
                self.gpu.resize(self.gpu.size);
            }
            // The system is out of memory, we should probably quit
            Err(wgpu::SurfaceError::OutOfMemory) => continue_render = false,
            _ => {}
        }

        self.interface.cleanup();
        self.game_manager.cleanup();

        (continue_render, cursor)
    }

    fn on_close(&mut self) {}

    fn focus_gained(&mut self) {}

    fn focus_lost(&mut self) {}

    fn on_resize(&mut self, size: Size, scale_factor: f64) {
        self.gpu.resize(size.into());
        self.interface.resize(size, scale_factor);
        self.game_manager.resize(size);
    }

    fn on_cursor_moved(&mut self, point: Point2D) {
        self.interface.set_cursor_position(point);
    }

    fn on_window_event(&mut self, event: Event) {
        self.interface.push_event(event);
    }

    fn on_mouse_moved(&mut self, x: f64, y: f64) {
        self.game_manager.on_mouse_moved(x, y);
    }

    fn should_cursor_grab(&mut self) -> bool {
        self.interface.should_grab_cursor()
    }

    fn on_keyboard_input(&mut self, state: ElementState, scancode: u32) {
        log::info!("{:?} {}", state, scancode);
        if state == ElementState::Pressed && scancode == 53 {
            self.interface.escape_pressed()
        }
    }
}

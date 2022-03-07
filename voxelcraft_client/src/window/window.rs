use crate::window::convert_window_event::convert_window_event;
use crate::window::EventHandler;
use iced::mouse::Interaction;
use std::error::Error;
use winit::dpi::{LogicalPosition, PhysicalPosition};
use winit::event::{DeviceEvent, Event, ModifiersState, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{CursorIcon, WindowBuilder};

pub struct Window {
    internal_window: winit::window::Window,
    event_loop: EventLoop<()>,
}

impl Window {
    pub fn create(title: &str) -> Result<Self, Box<dyn Error>> {
        log::info!("Creating new window with title {}", title);
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().with_title(title).build(&event_loop)?;

        Ok(Self {
            internal_window: window,
            event_loop,
        })
    }

    pub fn window(&self) -> &winit::window::Window {
        &self.internal_window
    }

    pub fn run<T: 'static + EventHandler>(mut self, mut event_handler: T) {
        log::info!("Starting event loop");

        let window = self.internal_window;
        let mut modifiers = ModifiersState::default();

        tokio::task::block_in_place(move || {
            self.event_loop
                .run(move |event, _target, mut control_flow| {
                    *control_flow = ControlFlow::Poll;
                    Self::handle_event(
                        &window,
                        event,
                        control_flow,
                        &mut event_handler,
                        &mut modifiers,
                    )
                });
        });
    }

    fn handle_event<T: EventHandler>(
        window: &winit::window::Window,
        event: Event<()>,
        mut control_flow: &mut ControlFlow,
        event_handler: &mut T,
        modifiers: &mut ModifiersState,
    ) {
        match event {
            Event::NewEvents(_) => {}
            Event::WindowEvent { window_id, event } if window_id == window.id() => {
                Self::handle_window_event(window, event, control_flow, event_handler, modifiers)
            }
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => {
                if event_handler.should_cursor_grab() {
                    event_handler.on_mouse_moved(delta.0, delta.1)
                }
            }
            Event::UserEvent(_) => {}
            Event::Suspended => {}
            Event::Resumed => {}
            Event::MainEventsCleared => {
                window
                    .set_cursor_grab(event_handler.should_cursor_grab())
                    .unwrap();
                window.set_cursor_visible(!event_handler.should_cursor_grab());
                window.request_redraw();
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let (should_continue, cursor) = event_handler.on_redraw_requested();

                if !should_continue {
                    *control_flow = ControlFlow::Exit;
                }

                let win_cursor = match cursor {
                    Interaction::Idle => CursorIcon::Default,
                    Interaction::Pointer => CursorIcon::Default,
                    Interaction::Grab => CursorIcon::Grab,
                    Interaction::Text => CursorIcon::Text,
                    Interaction::Crosshair => CursorIcon::Crosshair,
                    Interaction::Working => CursorIcon::Wait,
                    Interaction::Grabbing => CursorIcon::Grabbing,
                    Interaction::ResizingHorizontally => CursorIcon::RowResize,
                    Interaction::ResizingVertically => CursorIcon::RowResize,
                };

                window.set_cursor_icon(win_cursor)
            }
            _ => {}
        }
    }

    fn handle_window_event<T: EventHandler>(
        window: &winit::window::Window,
        event: WindowEvent,
        mut control_flow: &mut ControlFlow,
        event_handler: &mut T,
        modifiers: &mut ModifiersState,
    ) {
        match &event {
            WindowEvent::Resized(physical_size) => {
                let scale_factor = window.scale_factor();
                event_handler.on_resize((*physical_size).into(), scale_factor)
            }
            WindowEvent::Moved(_) => {}
            WindowEvent::CloseRequested => {
                event_handler.on_close();
                *control_flow = ControlFlow::Exit;
            }
            WindowEvent::Destroyed => {}
            WindowEvent::DroppedFile(_) => {}
            WindowEvent::HoveredFile(_) => {}
            WindowEvent::HoveredFileCancelled => {}
            WindowEvent::ReceivedCharacter(_) => {}
            WindowEvent::Focused(gained) => {
                if *gained {
                    event_handler.focus_gained();
                } else {
                    event_handler.focus_lost();
                }
            }
            WindowEvent::KeyboardInput { input, .. } => {
                event_handler.on_keyboard_input(input.state, input.scancode)
            }
            WindowEvent::ModifiersChanged(new_modifiers) => {
                *modifiers = *new_modifiers;
            }
            WindowEvent::CursorMoved { position, .. } => {
                event_handler.on_cursor_moved(position.to_logical(window.scale_factor()).into());
                if event_handler.should_cursor_grab() {
                    window.set_cursor_position(PhysicalPosition {
                        x: window.inner_size().width / 2,
                        y: window.inner_size().height / 2,
                    });
                };
            }
            WindowEvent::CursorEntered { .. } => {}
            WindowEvent::CursorLeft { .. } => {}
            WindowEvent::MouseWheel { .. } => {}
            WindowEvent::MouseInput { .. } => {}
            WindowEvent::TouchpadPressure { .. } => {}
            WindowEvent::AxisMotion { .. } => {}
            WindowEvent::Touch(_) => {}
            WindowEvent::ScaleFactorChanged {
                new_inner_size,
                scale_factor,
            } => event_handler.on_resize((**new_inner_size).into(), *scale_factor),
            WindowEvent::ThemeChanged(_) => {}
        }

        if let Some(e) = convert_window_event(&event, window.scale_factor(), *modifiers) {
            event_handler.on_window_event(e);
        }
    }
}

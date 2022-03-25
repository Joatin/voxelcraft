use crate::primitives::{Point2D, Size};
use iced::mouse::Interaction;
use iced_native::Event;
use winit::event::ElementState;

pub trait EventHandler {
    fn on_redraw_requested(&mut self) -> (bool, Interaction);
    fn on_close(&mut self);
    fn focus_gained(&mut self);
    fn focus_lost(&mut self);
    fn on_resize(&mut self, size: Size, scale_factor: f64);
    fn on_cursor_moved(&mut self, point: Point2D);
    fn on_window_event(&mut self, event: Event);
    fn on_mouse_moved(&mut self, x: f64, y: f64);
    fn should_cursor_grab(&mut self) -> bool;
    fn on_keyboard_input(&mut self, state: ElementState, scancode: u32);
}
